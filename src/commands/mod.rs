pub mod make;
pub mod show;
pub mod build;

/// General actions for each command
/// all the command need to extend from this
pub trait RxCommand {
    // #[allow(dead_code)]
    /// Function to execute the command
    fn exec();

    /// Show the help for the according command
    fn help() {
        println!("Help command for [command]");
    }
}