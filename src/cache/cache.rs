use crate::cache::storage_handler::storage_handler::*;
use crate::calendar::calendar::Calendar;
use std::error::Error;
use chrono;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Cache {
    pub cached_date: String,
    pub calendar: Calendar
}


pub async fn store(_calendar: Calendar) -> Result<(), Box<dyn Error>> {
    let date = chrono::Utc::now().format("%Y-%m-%d");
    let cache = Cache {
        cached_date: date.to_string(),
        calendar: _calendar
    };
    let json_string = serde_json::to_string(&cache).unwrap();
    write_to_cache(json_string).await?;
    Ok(())
}

pub async fn restore() -> Result<Cache, Box<dyn Error>> {
    let result = read_from_cache().await?;
    let cache = serde_json::from_str::<Cache>(result.as_str())?;
    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
    if !cache.cached_date.eq(&today) {
        Err("cache is invalid")?
    }
    Ok(cache)
}