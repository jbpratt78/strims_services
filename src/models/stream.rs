// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use chrono::NaiveDateTime;

#[derive(Queryable, Debug)]
pub struct Stream {
    pub id: Option<i32>,
    pub service: String,
    pub channel: String,
    pub path: Option<String>,
    pub hidden: Option<bool>,
    pub afk: Option<bool>,
    pub promoted: Option<bool>,
    pub title: String,
    pub thumbnail: Option<String>,
    pub live: Option<bool>,
    pub viewers: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
