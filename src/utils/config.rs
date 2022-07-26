use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub is_occasions_fetched: bool,
    pub fetched_occasions_year: Option<u16>,
    pub cached_month: Option<u8>,
}

impl Config {

    pub fn load() -> Self {
        Config {
            is_occasions_fetched: false,
            fetched_occasions_year: None,
            cached_month: None,
        }
    }

}