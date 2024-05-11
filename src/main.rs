use commands::{make::RxMake, RxCommand};

use crate::commands::show::RxShow;

mod commands;

fn main() {
    let command = std::env::args().nth(1).unwrap_or_else(|| String::from("help"));

    if command.contains("show") {
        RxShow::exec();
        RxShow::help();
    }

    if command.contains("make") {
        let name = std::env::args().nth(2).unwrap_or_else(|| String::new());

        if command.contains("make:api") && !name.is_empty() {
            RxMake::make_api(name);
            return;
        }

        if command.contains("make:page") && !name.is_empty() {
            RxMake::make_page(name);
            return;
        }

        RxMake::help();
    }
}
