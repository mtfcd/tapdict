use crate::utils::{ocr, os_utils, word};

use std::time::Instant;

pub fn get_word() -> Option<String> {
    let (img, pos) = os_utils::get_img_pos();
    let buf = img.buffer();
    let word_res = ocr::get_word(buf.to_vec(), pos);
    if let Err(e) = word_res {
        println!("get word error: {}", e.to_string());
        return None;
    }

    let word = word_res.unwrap();
    Some(word)
}

pub async fn get_def() -> Option<String> {
    let start = Instant::now();
    let word = get_word();
    if word.is_none() {
        return None;
    }

    let word = word.unwrap();
    println!("get word take: {:?} {}", start.elapsed(), &word);
    let def_res = word::lookup(&word).await;
    if let Err(e) = &def_res {
        println!("lookup error: {}", e);
        return None;
    }

    let def = def_res.unwrap();
    println!("{}", def);
    Some(def)
}

#[test]
pub fn get_word_use_tessract() {
    let (img, pos) = os_utils::get_img_pos();
    let buf = img.buffer();
    ocr::get_word(buf.to_vec(), pos).unwrap();
}
