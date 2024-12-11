use std::{env, process::exit};

use port_scanner::{args::Arguments, sniffer::Sniffer};

fn main() {
    let args: Vec<String> = env::args().collect();

    let parsed_args = match Arguments::new(&args) {
        Ok(val) => val,
        Err(e) => {
            if e == "Help flag passed" {
                println!(
                    "Usage: port_scanner [OPTIONS]

A fast port scanner with IP sweep capabilities.

Options:
    -p, --ports      Specify ports to scan (e.g., -p=8080,8081)
    -j, --jobs       Number of parallel jobs (default: 100)
    -h, --help       Display this help message

IP Sweep Support:
    Supports wildcard (*) notation for one octet
    Example: 192.168.*.1 will scan 192.168.0.1 through 192.168.255.1

Examples:
    port_scanner -p=80,443 192.168.1.1
    port_scanner -p=22,80 -j=50 192.168.0.1

NOTE: IP Sweep does not supports specific ports"
                );
                exit(0);
            } else {
                eprintln!("Error: {e}");
                exit(1);
            }
        }
    };
    if parsed_args.is_sweep_scan() {
        let active_ips = Sniffer::sweep_scan(parsed_args);

        println!("Active IPs:");
        for ip in active_ips.list() {
            println!("{ip}");
        }
    } else {
        let open_ports = Sniffer::scan(&parsed_args);
        if parsed_args.is_result_print() {
            println!("Open: {:?}", open_ports.list());
        } else {
            for port in open_ports.list() {
                println!("Open = {}:{}", parsed_args.host, port)
            }
        }
    }
}
