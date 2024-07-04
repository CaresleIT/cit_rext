use std::{fs::{self, File}, io::Write, path::Path};

pub struct RxFileManager {}

impl RxFileManager {
    /// Save the file in the given path, creates the 
    /// folders in case of not exist
    pub fn save_file(path: String, file: &str, content: String) -> Option<bool> {
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

        let file_path = format!("{}/{}", final_path_clone, file);
        let exists = Path::new(&file_path).exists();

        if exists {
            return Some(true);
        }
        let file = File::create(format!("{}/{}", final_path_clone, file)); 

        match file {
            Ok(mut f) => {
                println!("File created");
                f.write_all(content.as_bytes()).expect("Unable to write data for file the selected file");
            },
            Err(e) => println!("{:?}", e),
        }
        Some(true)
    }
}