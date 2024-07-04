use commands::{build::logic_command_build, help::RxHelp, make::logic_command_make, show::logic_command_show, RxCommand};

mod commands;
mod file;
mod templates;

fn main() {
    let command = std::env::args().nth(1).unwrap_or_else(|| String::from(""));

    logic_command_show(&command);
    logic_command_make(&command);
    logic_command_build(&command);

    if command.contains("help") || command.is_empty() {
        RxHelp::exec();
    }
}