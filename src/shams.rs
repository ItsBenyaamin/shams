use crate::cache::cache::*;
use crate::calendar::calendar::Calendar;
use crate::consts::consts::TIME_URL;
use crate::output::body_parser as parser;
use crate::output::printer::printer::*;

pub async fn get(url: &str) -> crate::Result<String> {
    let resp = reqwest::get(url).await?.text().await?;
    Ok(resp)
}

pub async fn go_with_cache() -> crate::Result<()> {
    let cache_result = Cache::restore().await;
    match cache_result {
        Ok(cache) => print(&cache.calendar),
        Err(_) => request().await?,
    }
    Ok(())
}

pub async fn request() -> crate::Result<()> {
    let response = get(TIME_URL).await?;
    let calendar = parser::process_the_body(response).await;
    print(&calendar);
    cache_calender(calendar).await?;
    Ok(())
}

async fn cache_calender(calendar: Calendar) -> crate::Result<()> {
    let cache = Cache::new(calendar);
    cache.store().await?;
    Ok(())
}
