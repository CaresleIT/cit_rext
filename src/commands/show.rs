use super::RxCommand;

pub struct RxShow {}

impl RxCommand for RxShow {
    fn exec() {
        println!("Rx Show command");
    }
}