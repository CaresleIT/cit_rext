use super::RxCommand;

/// Show command for listing all routes
/// in the app, either of the `api` endpoints or the
/// `web` part of the application
pub struct RxShow {}

impl RxCommand for RxShow {
    fn exec() {
        println!("Rx Show command");
    }

    fn help() {
        println!("Rx Show help");
    }
}

pub fn logic_command_show(command: &String) {    
    if command.contains("show") {
        RxShow::exec();
        RxShow::help();
    }
}
