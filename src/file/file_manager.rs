use std::fs::File;

pub struct RxFileManager {
}

impl RxFileManager {
    pub fn save_file(path: String) -> Option<bool> {
        let file = File::create(path);

        match file {
            Ok(_) => println!("File created"),
            Err(e) => println!("{:?}", e),
        }
        Some(true)
    }
}