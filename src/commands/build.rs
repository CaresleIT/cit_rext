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