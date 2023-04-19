use crate::utils::{ocr, os_utils};

pub async fn get_word() -> Option<String> {
    let (img, pos) = os_utils::get_img_pos();
    let buf = img.buffer();
    let word_res = ocr::get_word(buf.to_vec(), pos).await;
    if let Err(e) = word_res {
        error!("get word error: {}", e.to_string());
        return None;
    }

    let word = word_res.unwrap();
    Some(word)
}

#[test]
pub async fn get_word_use_tessract() {
    let (img, pos) = os_utils::get_img_pos();
    let buf = img.buffer();
    ocr::get_word(buf.to_vec(), pos).await.unwrap();
}
