
use crate::{file::file_manager::RxFileManager, templates::{api_template, queries_template, route_template, schema_template}};

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
        let path_to_save = format!("./app/api/{}", name);

        RxFileManager::save_file(String::from(&path_to_save), "route.js", api_template());
        RxFileManager::save_file(String::from(&path_to_save), "schema.js", schema_template());
        RxFileManager::save_file(String::from(&path_to_save), "queries.js", queries_template());
    }

    pub fn make_page(name: String, options: String) {
        println!("We will create a page with the name {}", name);
        println!("These are the options passed to the command {}", options);

        let path_to_save = format!("./app/{}", name);
        
        let string_page = route_template(&format!("Page{}", ""));
        let string_loading = route_template(&format!("Loading{}", ""));
        let string_error = route_template(&format!("Error{}", ""));
        let string_layout = route_template(&format!("Layout{}", ""));
        let string_default = route_template(&format!("Default{}", ""));
        let string_not = route_template(&format!("NotFound{}", ""));

        // Save the files
        RxFileManager::save_file(String::from(&path_to_save), "page.jsx",string_page);
        RxFileManager::save_file(String::from(&path_to_save), "loading.jsx", string_loading);
        RxFileManager::save_file(String::from(&path_to_save), "error.jsx", string_error);
        RxFileManager::save_file(String::from(&path_to_save), "layout.jsx", string_layout);
        RxFileManager::save_file(String::from(&path_to_save), "default.jsx", string_default);
        RxFileManager::save_file(String::from(&path_to_save), "not-found.jsx", string_not);
        RxFileManager::save_file(String::from(format!("{}/_components", path_to_save)), "", String::new());
        RxFileManager::save_file(String::from(format!("{}/_helpers", path_to_save)), "", String::new());
        RxFileManager::save_file(String::from(format!("{}/_hooks", path_to_save)), "", String::new());
        RxFileManager::save_file(String::from(format!("{}/_states", path_to_save)), "", String::new());
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