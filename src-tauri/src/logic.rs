use crate::utils::{ocr, os_utils};

pub async fn get_word() -> Option<String> {
    let (img, pos) = os_utils::get_img_pos();
    let word_res = ocr::get_word(img, pos).await;
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
    ocr::get_word(img, pos).await.unwrap();
}
