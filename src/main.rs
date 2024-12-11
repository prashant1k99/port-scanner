use std::{env, process::exit};

use port_scanner::{args::Arguments, sniffer::Sniffer};

fn main() {
    let args: Vec<String> = env::args().collect();

    let parsed_args = match Arguments::new(&args) {
        Ok(val) => val,
        Err(e) => {
            if e == "Help flag passed" {
                // TODO: handle help flag content
                println!("Asking for help");
                exit(0);
            } else {
                eprintln!("Error: {e}");
                exit(1);
            }
        }
    };
    if parsed_args.is_sweep_scan() {
    } else {
        let open_ports = Sniffer::scan(&parsed_args).unwrap();

        for port in open_ports.list() {
            println!("Open = {}:{}", parsed_args.host, port)
        }
    }
}
