use std::fs;

pub struct RxFileManager {
}

impl RxFileManager {
    /// Save the file in the given path, creates the 
    /// folders in case of not exist
    pub fn save_file(path: String) -> Option<bool> {
        let file = fs::create_dir_all(path);

        match file {
            Ok(_) => println!("File created"),
            Err(e) => println!("{:?}", e),
        }
        Some(true)
    }
}