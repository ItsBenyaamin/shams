use crate::shams::occasions::Occasions;
use crate::utils::constants;

pub async fn fetch_occasions(year: i32) -> Option<Vec<Occasions>> {
    let url = format!("{}/{}", constants::OCCASIONS_URL, year);
    let response = reqwest::get(url.as_str())
        .await;
    if response.is_ok() && response.as_ref().unwrap().status().is_success() {
        let to_json: reqwest::Result<Option<Vec<Occasions>>> = response.unwrap().json().await;
        if to_json.is_ok() && to_json.as_ref().unwrap().is_some() {
            return to_json.unwrap();
        }
    }

    None
}


#[cfg(test)]
mod tests {
    use crate::Occasions;
    use crate::server::fetch_occasions;

    #[tokio::test]
    async fn fail_case_when_offline() {
        let oc: Vec<Occasions> = Vec::new();
        let occasions = fetch_occasions(0).await;
        assert_eq!(occasions, None);
    }

}