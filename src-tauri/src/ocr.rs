use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use opencv::core::{self, Point2f, Scalar, Size, Vector};
use opencv::prelude::*;
use opencv::types::{VectorOfPoint, VectorOfPoint2f, VectorOfString, VectorOfVectorOfPoint};
use opencv::{dnn, imgcodecs, imgproc};

use anyhow::{Error, Result};

fn if_point_in_region(pos: [i32; 2], region: &VectorOfPoint) -> bool {
    let region_slice = region.as_slice();
    let top_left = region_slice[1];
    let bottom_right = region_slice[3];

    let x = pos[0];
    let y = pos[1];
    x > top_left.x && y > top_left.y && x < bottom_right.x && y < bottom_right.y
}

pub fn extract_word(buf: Vec<u8>, pos: [i32; 2]) -> Result<String> {
    let imread_rgb = false;
    let arr: Vector<u8> = Vector::from(buf);
    let img = imgcodecs::imdecode(&arr, imgcodecs::IMREAD_COLOR)?;

    let det_results = detect_text_region(&img)?;
    let rec_input = if !imread_rgb {
        let mut rec_input = Mat::default();
        imgproc::cvt_color(&img, &mut rec_input, imgproc::COLOR_BGR2GRAY, 0)?;
        Some(rec_input)
    } else {
        None
    };

    let region = det_results
        .iter()
        .find(|quadrangle| if_point_in_region(pos, &quadrangle))
        .ok_or(Error::msg("no word"))?;
    let mut quadrangle_2f = VectorOfPoint2f::new();
    for pt in &region {
        quadrangle_2f.push(Point2f::new(pt.x as f32, pt.y as f32))
    }
    let cropped =
        four_points_transform(rec_input.as_ref().unwrap_or(&img), quadrangle_2f.as_slice())?;

    recognize(cropped)
}

lazy_static! {
    static ref DETECTOR: Arc<Mutex<dnn::TextDetectionModel_EAST>> = {
        let conf_threshold = 0.5;
        let nms_threshold = 0.4;
        let width = 320;
        let height = 320;
        let det_model_path =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("resources").join("frozen_east_text_detection.pb");
        let mut d =
            dnn::TextDetectionModel_EAST::from_file(det_model_path.to_str().unwrap(), "").unwrap();
        d
            .set_confidence_threshold(conf_threshold).unwrap()
            .set_nms_threshold(nms_threshold).unwrap();
        // Parameters for Detection
        let det_scale = 1.;
        let det_input_size = Size::new(width, height);
        let det_mean = Scalar::from((123.68, 116.78, 103.94));
        let swap_rb = true;
        d.set_input_params(det_scale, det_input_size, det_mean, swap_rb, false).unwrap();
        Arc::new(Mutex::new(d))
    };

    static ref RECOGNIZER: Arc<Mutex<dnn::TextRecognitionModel>> = {
    // Parameters.
    let rec_model_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("resources").join("CRNN_VGG_BiLSTM_CTC_float16.onnx");
    let voc_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("resources").join("alphabet_36.txt");

    // Load networks.
    let mut r =
        dnn::TextRecognitionModel::from_file(rec_model_path.to_str().unwrap(), "").unwrap();

    // Load vocabulary
    let mut vocabulary = VectorOfString::new();
    let voc_file = BufReader::new(File::open(voc_path).unwrap());
    for voc_line in voc_file.lines() {
        vocabulary.push(&voc_line.unwrap());
    }
    r
        .set_vocabulary(&vocabulary).unwrap()
        .set_decode_type("CTC-greedy").unwrap();

    // Parameters for Recognition
    let rec_scale = 1. / 127.5;
    let rec_mean = Scalar::from((127.5, 127.5, 127.5));
    let rec_input_size = Size::new(100, 32);
    r.set_input_params(rec_scale, rec_input_size, rec_mean, false, false).unwrap();
     Arc::new(Mutex::new(r))
    };
}

pub fn detect_text_region(img: &Mat) -> Result<VectorOfVectorOfPoint> {
    // Detection
    let mut det_results = VectorOfVectorOfPoint::new();
    DETECTOR.lock().unwrap().detect(img, &mut det_results)?;

    return Ok(det_results);
}

pub fn recognize(img: Mat) -> Result<String> {
    let recognition_result = RECOGNIZER.lock().unwrap().recognize(&img)?;
    return Ok(recognition_result);
}

fn four_points_transform(frame: &Mat, vertices: &[Point2f]) -> Result<Mat> {
    let output_size = Size::new(100, 32);
    let target_vertices = [
        Point2f::new(0., (output_size.height - 1) as f32),
        Point2f::new(0., 0.),
        Point2f::new((output_size.width - 1) as f32, 0.),
        Point2f::new(
            (output_size.width - 1) as f32,
            (output_size.height - 1) as f32,
        ),
    ];
    let rotation_matrix =
        imgproc::get_perspective_transform_slice(vertices, &target_vertices, core::DECOMP_LU)?;
    let mut out = Mat::default();
    imgproc::warp_perspective(
        frame,
        &mut out,
        &rotation_matrix,
        output_size,
        imgproc::INTER_LINEAR,
        core::BORDER_CONSTANT,
        Scalar::default(),
    )?;
    Ok(out)
}
