use super::RxCommand;

pub struct RxBuild {}

impl RxCommand for RxBuild {
    fn exec() {
        println!("build command");
    }

    fn help() {
        println!("help of build");
    }
}

pub fn logic_command_build(command: &String) {
    if command.contains("build") {
        RxBuild::exec();
    }
}