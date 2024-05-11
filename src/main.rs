use commands::RxCommand;

use crate::commands::show::RxShow;

mod commands;

fn main() {
  let command = std::env::args().nth(1);

  match command {
      Some(n) => {
        if n.contains("show") {
            RxShow::exec();
        }
      },
      None => println!("Please provide an option")
  }
}
