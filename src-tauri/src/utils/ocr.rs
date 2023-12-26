use anyhow::{Error, Result};
use regex::Regex;
use std::sync::{Arc, Mutex};
use tesseract::Tesseract;

lazy_static! {
    pub static ref TESSDATA_DIR: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
}

pub async fn get_word(buf: Vec<u8>, pos: (i32, i32)) -> Result<String> {
    let mut tes = Tesseract::new(TESSDATA_DIR.lock().unwrap().as_deref(), Some("eng"))
        .map_err(|e| {
            println!("init tes err: {}", e);
            e
        })
        .unwrap()
        .set_image_from_mem(&buf)
        .map_err(|e| {
            println!("reg tes err: {}", e);
            e
        })
        .unwrap();

    let tsv = tes.get_tsv_text(1).unwrap();
    find_word_in_pos(&tsv, pos)
}

fn extract_single_word(line_part: &str, x: i32, width: i32) -> &str {
    debug!("extract word: {}, {}, {}", line_part, x, width);
    lazy_static! {
        static ref PAT: Regex = Regex::new(r"[\W0-9_]").unwrap();
    }
    if !PAT.is_match(line_part) {
        return line_part;
    }
    let estimate = line_part.len() * x as usize / width as usize;
    let mut acc = 0;
    for seg in PAT.split(line_part) {
        let acc_word_len = seg.len() + acc;
        if acc_word_len > estimate {
            return seg;
        }
        acc = acc_word_len + 1;
    }
    line_part
}

fn find_word_in_pos(tsv: &str, pos: (i32, i32)) -> Result<String> {
    for line in tsv.lines() {
        debug!("tsv line: {line}");
        let line_parts: Vec<&str> = line.split('\t').collect();
        if line_parts.len() != 12 {
            return Err(Error::msg("tsv err"));
        }
        let left: i32 = line_parts[6].parse()?;
        let top: i32 = line_parts[7].parse()?;
        let width: i32 = line_parts[8].parse()?;
        let height: i32 = line_parts[9].parse()?;
        let conf: f32 = line_parts[10].parse()?;

        let x = pos.0 - left;
        let y = pos.1 - top;
        if 0 < x && x < width && 0 < y && y < height && conf > 0.0 {
            let word = extract_single_word(line_parts[11], x, width);
            return Ok(word.to_lowercase());
        }
    }
    Err(Error::msg("tsv word not found"))
}

#[test]
fn ocr_large_word() {
    let frame = include_bytes!("screen.png");
    let word = get_word(frame.to_vec(), (100, 50)).await.unwrap();
    assert_eq!("Community", word);
}

#[test]
fn test_extract_singal_word() {
    let line = "arts[8].parse()?;";
    let x = 100;
    let width = 148;
    let word = extract_single_word(line, x, width);
    assert_eq!("parse", word);
}
