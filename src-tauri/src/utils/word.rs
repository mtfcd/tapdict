use anyhow::{Error, Result};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::include_str;

const API_URL: &'static str = include_str!("api_key");

#[derive(Serialize, Deserialize)]
struct AppShortdef {
    hw: String,
    fl: String,
    def: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Meta {
    #[serde(rename = "app-shortdef")]
    app_shortdef: AppShortdef,
}

#[derive(Serialize, Deserialize, Default)]
struct Sound {
    audio: String,
}

#[derive(Serialize, Deserialize)]
struct Pronouce {
    ipa: String,
    #[serde(default)]
    sound: Sound,
}

#[derive(Serialize, Deserialize)]
struct HeadWordInfo {
    hw: String,
    #[serde(default)]
    prs: Vec<Pronouce>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Word {
    meta: Meta,
    hwi: HeadWordInfo,
}

async fn req_api(word: &str) -> Result<Value> {
    let res = reqwest::get(format!(
        "https://dictionaryapi.com/api/v3/references/learners/json/{}?key={}",
        word, API_URL
    ))
    .await?;

    let txt = res.text().await?;
    debug!("get api res: {}", &txt);
    let v: Value = serde_json::from_str(&txt)?;
    Ok(v)
}

pub async fn lookup(word: &str) -> Result<String> {
    let mut res = req_api(word).await?;
    let def_value = loop {
        if res.is_object() {
            break res;
        }
        if !res.is_array() {
            return Err(Error::msg("wrong type"));
        }
        let arr = res.as_array().unwrap();
        let ele = &arr[0];
        if ele.is_object() {
            break ele.clone();
        }
        if !ele.is_string() {
            return Err(Error::msg("wrong type"));
        }
        debug!("retry {:?}", arr);
        res = req_api(&ele.as_str().unwrap()).await?
    };

    let def: Word = serde_json::from_value(def_value)?;
    Ok(serde_json::to_string_pretty(&def)?)
}

#[test]
fn test_lookup() {
    tauri::async_runtime::block_on(async {
        println!("test BCE");
        let res = lookup("BCE").await.unwrap();
        println!("{}", res);
    });
}
