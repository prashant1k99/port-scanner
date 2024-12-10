use std::{env, process::exit};

use port_scanner::args::Arguments;

fn main() {
    let args: Vec<String> = env::args().collect();

    let parsed_args = Arguments::new(&args);
    if let Err(e) = parsed_args {
        if e == "Help flag passed" {
            // TODO: handle help flag content
            println!("Asking for help");
            exit(0);
        } else {
            eprintln!("Error: {e}");
            exit(1);
        }
    }

    println!("{parsed_args:?}");
}
