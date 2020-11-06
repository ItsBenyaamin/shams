use crate::cache::storage_handler::*;
use crate::calendar::calendar::Calendar;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Cache {
    pub cached_date: String,
    pub calendar: Calendar,
}

impl Cache {
    pub fn new(calendar: Calendar) -> Self {
        Cache {
            cached_date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
            calendar,
        }
    }

    pub async fn store(&self) -> crate::Result<()> {
        write_to_cache(serde_json::to_string(&self)?).await?;
        Ok(())
    }

    pub async fn restore() -> crate::Result<Cache> {
        let result = read_from_cache().await?;
        let cache = serde_json::from_str::<Cache>(result.as_str())?;
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
        if !cache.cached_date.eq(&today) {
            return Err("cache is invalid".into());
        }
        Ok(cache)
    }
}
