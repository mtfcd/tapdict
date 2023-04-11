use anyhow::{Error, Result};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::SqliteConnection;
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
struct MwWord {
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

fn mw_prs_to_prs(mw: &Pronouce) -> Prs {
    Prs {
        ipa: Some(mw.ipa.to_owned()),
        audio: Some(mw.sound.audio.to_owned()),
    }
}

async fn lookup_from_mw(word: &str) -> Result<Word> {
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

    let def: MwWord = serde_json::from_value(def_value)?;
    Ok(Word {
        hw: def.meta.app_shortdef.hw,
        def: def.meta.app_shortdef.def,
        trans: vec![],
        prs: def.hwi.prs.iter().map(mw_prs_to_prs).collect(),
    })
}

pub async fn lookup(word: &str, conn: Option<&mut SqliteConnection>) -> Result<String> {
    if let Some(c) = conn {
        let def = lookup_from_local(word, c).await;
        if def.is_ok() {
            return Ok(serde_json::to_string_pretty(&def?)?);
        } else {
            error!("look up local for: {}, error: {:?}", word, def.err());
        }
    }
    let def = lookup_from_mw(word).await?;
    Ok(serde_json::to_string_pretty(&def)?)
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Prs {
    ipa: Option<String>,
    audio: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Word {
    hw: String,
    def: Vec<String>,
    trans: Vec<String>,
    prs: Vec<Prs>,
}

use crate::utils::word_local;

fn chunck_def(def: Option<String>) -> Vec<String> {
    if def.is_none() {
        return vec![];
    }
    def.unwrap().lines().map(|l| l.to_string()).collect()
}

pub async fn lookup_from_local(word: &str, conn: &mut SqliteConnection) -> Result<Word> {
    let word = word_local::lookup(word, conn).await.map(|d| Word {
        hw: d.word,
        def: chunck_def(d.definition),
        trans: chunck_def(d.translation),
        prs: vec![Prs {
            ipa: d.phonetic,
            audio: None,
        }],
    })?;
    Ok(word)
}

#[test]
fn test_lookup() {
    tauri::async_runtime::block_on(async {
        println!("test BCE");
        let res = lookup("BCE", Default::default()).await.unwrap();
        println!("{}", res);
    });
}
