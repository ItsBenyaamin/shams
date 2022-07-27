use std::path::PathBuf;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::utils::constants;


pub async fn read_file(file_name: &str) -> Option<String> {
    if let Some(file) = get_file(file_name).await {
        let mut file_options = OpenOptions::new().read(true).open(file).await.unwrap();
        let mut buff = String::new();
        file_options.read_to_string(&mut buff).await.unwrap();
        return Some(buff);
    }

    None
}

pub async fn write_to_file(file_name: &str, cache_content: String) {
    if let Some(file) = get_file(file_name).await {
        let mut file_options = OpenOptions::new().write(true).open(file).await;
        if file_options.is_ok() {
            let _ = file_options.unwrap().write_all(cache_content.as_bytes()).await;
        }
    }
}

async fn get_file(file_name: &str) -> Option<PathBuf> {
    let config_folder = match dirs::config_dir() {
        Some(c) => c.to_string_lossy().to_string(),
        None => {
            return None;
        }
    };

    let mut path = PathBuf::from(config_folder);
    path = path.join(constants::SHAMS_FOLDER_NAME);
    if !path.exists() {
        std::fs::create_dir(&path).unwrap();
    }
    path.push(file_name);
    if !path.exists() {
        std::fs::File::create(&path).unwrap();
    }
    Some(path)
}

pub async fn delete_file(file_name: &str) {
    if let Some(file) = get_file(file_name).await {
        tokio::fs::remove_file(file).await.unwrap_or_default();
    }
}