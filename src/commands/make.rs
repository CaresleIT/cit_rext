
use crate::file::file_manager::RxFileManager;

use super::RxCommand;

pub struct RxMake {}

impl RxCommand for RxMake {
    fn exec() {
        print!("Make command");
    }

    fn help() {
        println!("Make command, please use the next options to use this command");
        println!();
        println!("make:api \t Generates an api route");
        println!("make:page \t Generates a page route (page, loading, error, layout)");
    }
}

impl RxMake {
    pub fn make_api(name: String, options: String) {
        println!("We will create an api route called {}", name);
        println!("These are the options passed to the command {}", options);

        // Creating the file

        RxFileManager::save_file(String::from(format!("./app/{}.jsx", name)));
    }

    pub fn make_page(name: String, options: String) {
        println!("We will create a page with the name {}", name);
        println!("These are the options passed to the command {}", options);
    }
}

pub fn logic_command_make(command : &String) {
    if command.contains("make") {
        let name = std::env::args().nth(2).unwrap_or_else(|| String::new());
        let options = std::env::args().nth(3).unwrap_or_else(|| String::new());

        if command.contains("make:api") && !name.is_empty() {
            RxMake::make_api(name, options);
            return;
        }

        if command.contains("make:page") && !name.is_empty() {
            RxMake::make_page(name, options);
            return;
        }

        RxMake::help();
    }
}