use serde::{Serialize, Deserialize};
use crate::utils::{constants, storage_helper};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
    pub is_occasions_fetched: bool,
    pub fetched_occasions_year: Option<u16>,
    pub cached_month: Option<u8>,
}

impl Config {

    fn new() -> Self {
        Config {
            is_occasions_fetched: false,
            fetched_occasions_year: None,
            cached_month: None,
        }
    }

    pub async fn load() -> Self {
        let file_content = storage_helper::read_file(constants::CONFIG_FILE_NAME).await;
        if let Some(content) = file_content {
            if !content.is_empty() {
                let config: Config = serde_json::from_str(&content).unwrap();
                return config;
            }
        }

        Self::new()
    }

}

impl ToString for Config {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}


#[cfg(test)]
mod test {
    use crate::Config;
    use crate::utils::{constants, storage_helper};

    #[tokio::test]
    async fn test_config_without_exist() {
        let config = Config::load().await;
        assert_eq!(config, Config::new())
    }

    #[tokio::test]
    async fn test_config_with_file_exist() {
        let mut config = Config::new();
        config.is_occasions_fetched = false;
        config.fetched_occasions_year = None;
        config.cached_month = None;

        storage_helper::write_to_file(constants::CONFIG_FILE_NAME, config.to_string()).await;

        let config = Config::load().await;

        assert_eq!(config, Config::new());

        storage_helper::delete_file(constants::CONFIG_FILE_NAME).await;
    }

}