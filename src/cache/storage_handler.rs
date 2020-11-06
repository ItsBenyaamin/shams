use crate::consts::consts::{CACHE_FILE_NAME, CACHE_FOLDER_PATH};
use std::path::PathBuf;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

fn get_file_path() -> crate::Result<PathBuf> {
    let cache_folder = dirs::cache_dir();
    let cache_folder = match cache_folder {
        Some(c) => c.to_string_lossy().to_string(),
        None => {
            return Err("get file path failed".into());
        }
    };

    let mut path = PathBuf::from(cache_folder);
    path = path.join(CACHE_FOLDER_PATH);
    if !path.exists() {
        std::fs::create_dir(&path)?;
    }
    path.push(CACHE_FILE_NAME);
    if !path.exists() {
        std::fs::File::create(&path)?;
    }
    Ok(path)
}

pub async fn read_from_cache() -> crate::Result<String> {
    let file_path: PathBuf = get_file_path()?;
    let mut file_options = OpenOptions::new().read(true).open(file_path).await?;
    let mut buff = String::new();
    file_options.read_to_string(&mut buff).await?;
    Ok(buff)
}

pub async fn write_to_cache(cache_content: String) -> crate::Result<()> {
    let file_path = get_file_path()?;
    let mut file_options = OpenOptions::new().write(true).open(file_path).await?;
    file_options.write_all(cache_content.as_bytes()).await?;
    Ok(())
}
