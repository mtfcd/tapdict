use anyhow::{Error, Result};
use std::path::Path;
use tesseract::{OcrEngineMode, Tesseract};

lazy_static! {
    static ref TESSDATA_DIR: &'static Path = Path::new("./resources");
}

pub fn get_word(buf: Vec<u8>, pos: (i32, i32)) -> Result<String> {
    let mut tes =
        Tesseract::new_with_oem(TESSDATA_DIR.to_str(), Some("eng"), OcrEngineMode::Default)
            .unwrap()
            .set_image_from_mem(&buf)
            .unwrap();

    let tsv = tes.get_tsv_text(1).unwrap();
    let word = find_word_in_pos(&tsv, pos);
    word
}

fn find_word_in_pos(tsv: &str, pos: (i32, i32)) -> Result<String> {
    for line in tsv.lines() {
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
            return Ok(line_parts[11].to_lowercase());
        }
    }
    return Err(Error::msg("tsv word not found"));
}
