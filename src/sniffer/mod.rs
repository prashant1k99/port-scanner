use std::net::{IpAddr, TcpStream};
use std::sync::mpsc;

use crate::args::Arguments;
use crate::workers::ThreadPool;

#[cfg(test)]
mod tests;

pub struct Sniffer {}

pub struct OpenPorts {
    open_ports: Vec<i32>,
}

impl OpenPorts {
    pub fn list(&self) -> &Vec<i32> {
        &self.open_ports
    }
}

pub struct ActiveIps {
    active_ips: Vec<IpAddr>,
}

impl ActiveIps {
    pub fn list(&self) -> &Vec<IpAddr> {
        &self.active_ips
    }
}

// We pass the args
// 2 functions in total, 1 is scan and 2nd is sweep_scan
// sweep_scan will return the active ip addresses

impl Sniffer {
    pub fn scan(ip_args: &Arguments) -> Result<OpenPorts, &'static str> {
        println!("Need to scan for: {ip_args:?}");

        let threads = ip_args.get_thread_counts().unwrap_or(100);
        let ports = match ip_args.get_port() {
            Some(val) => val,
            None => (1..=65535).collect(),
        };

        let pool = ThreadPool::new(threads);

        // Create a response channel, which will keep on writing response with port
        let (tx, rx) = mpsc::channel();

        let ip = ip_args.host;

        for port in ports {
            let tx_i = tx.clone();
            pool.execute(move || is_port_open(ip, port, tx_i));
        }

        drop(tx);

        let mut open_ports: Vec<i32> = vec![];
        for response in rx {
            open_ports.push(response)
        }

        Ok(OpenPorts { open_ports })
    }

    pub fn sweep_scan(ip_args: Arguments) -> Result<ActiveIps, &'static str> {
        println!("Need to scan for: {ip_args:?}");
        // Try to do the sweep scan
        Ok(ActiveIps { active_ips: vec![] })
    }
}

fn is_port_open(ip: IpAddr, port: i32, tx: mpsc::Sender<i32>) {
    // println!("Scanning {ip}:{port}");
    if TcpStream::connect(format!("{}:{}", ip, port)).is_ok() {
        tx.send(port).unwrap();
    }
}
