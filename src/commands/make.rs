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
    pub fn make_api(name: String) {
        println!("We will create an api route called {}", name);
    }

    pub fn make_page(name: String) {
        println!("We will create a page with the name {}", name);
    }
}