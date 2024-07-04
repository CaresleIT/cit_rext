use super::RxCommand;

pub struct RxHelp {}

impl RxCommand for RxHelp {
    fn exec() {
        println!("=== RxCLI ===");
        println!("Unopinionated CLI for nextjs, inspired in laravel artisan cli.");
        println!("\n");
        println!("Commands");
        println!("\tmake\tFor make routes or api routes files");
        println!("\tbuild\tFor export the naviagation routes to a file");
        println!("\tshow\tFor showing a list of all the routes in the app");
        println!("\thelp\tShow the welcome view");
    }

    fn help() {
        RxHelp::exec();
    }
}