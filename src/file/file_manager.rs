use std::fs::{self, File};

pub struct RxFileManager {
}

impl RxFileManager {
    /// Save the file in the given path, creates the 
    /// folders in case of not exist
    pub fn save_file(path: String, file: &str) -> Option<bool> {
        let mut final_path = path;

        if final_path.contains("//") {
            final_path = final_path.replace("//", "/").to_string();
        }

        let final_path_clone = final_path.clone();
        let folder = fs::create_dir_all(final_path);

        match folder {
            Ok(_) => println!("Folders created"),
            Err(e) => println!("{:?}", e),
        }

        if file.is_empty() {
            return Some(true);
        }

        let file = File::create(format!("{}{}", final_path_clone, file)); 

        match file {
            Ok(_) => println!("File created"),
            Err(e) => println!("{:?}", e),
        }
        Some(true)
    }
}