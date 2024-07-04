pub mod file_manager;

#[cfg(test)]
mod tests {
    use std::path::Path;

    use self::file_manager::RxFileManager;

    use super::*;

    #[test]
    fn test_empty_values() {
        let path = String::new();
        let file = "";
        let content = String::new();

        RxFileManager::save_file(path, file, content);
    }

    #[test]
    fn test_create_simple_file() {
        let path = String::from("./app/");
        let file = "value.txt";
        let content = String::from("Content");

        RxFileManager::save_file(path, file, content);
        assert_eq!(Path::new("./app/value.txt").exists(), true);
    }
}