pub mod storage_handler {
    use std::path::PathBuf;
    use std::io::{Write, Read};
    use std::fs::OpenOptions;

    const CACHE_FOLDER_PATH: &str = "/shams/";
    const CACHE_FILE_NAME: &str = "shams.txt";

    fn get_file_path() -> PathBuf {
        let cache_folder = dirs::cache_dir().unwrap();
        let mut folder_path = PathBuf::new();
        folder_path.push(cache_folder.as_path().to_string_lossy().to_string() + CACHE_FOLDER_PATH);
        if !folder_path.exists() {
            std::fs::create_dir(&folder_path).unwrap();
        }
        let file_path_string = folder_path.to_string_lossy().to_string() + CACHE_FILE_NAME;
        let mut file_path = PathBuf::new();
        file_path.push(file_path_string);
        if !file_path.exists() {
            std::fs::File::create(&file_path).unwrap();
        }
        file_path
    }

    pub async fn read_from_cache() -> Result<String, Box<dyn std::error::Error>> {
        let file_path: PathBuf = get_file_path();
        let mut file_options = OpenOptions::new().read(true).open(file_path).unwrap();
        let mut buff = String::new();
        file_options.read_to_string(&mut buff).unwrap();
        Ok(buff)
    }

    pub async fn write_to_cache(cache_content: String) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = get_file_path();
        let mut file_options = OpenOptions::new().write(true).open(file_path).unwrap();
        file_options.write_all(cache_content.as_bytes())?;
        Ok(())
    }

}