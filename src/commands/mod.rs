pub mod make;
pub mod show;

/// General actions for each command
/// all the command need to extend from this
pub trait RxCommand {
    // #[allow(dead_code)]
    /// Function to execute the command
    fn exec();
}