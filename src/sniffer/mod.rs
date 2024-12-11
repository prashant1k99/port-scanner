use std::net::{IpAddr, Ipv4Addr, TcpStream};
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
    pub fn scan(ip_args: &Arguments) -> OpenPorts {
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

        OpenPorts { open_ports }
    }

    pub fn sweep_scan(ip_args: Arguments) -> ActiveIps {
        let threads = ip_args.get_thread_counts().unwrap_or(100);
        let ip_range = Self::generate_ip_range(ip_args.host, ip_args.sweep_octact().unwrap());
        // TO get the range. Using Octat and the host we are going to identify the last point of
        // the range

        let pool = ThreadPool::new(threads);

        // Create a response channel, which will keep on writing response with port
        let (tx, rx) = mpsc::channel();

        for ip in ip_range {
            let tx_i = tx.clone();
            pool.execute(move || scan_for_ip(ip, tx_i));
        }

        drop(tx);

        let mut active_ips: Vec<IpAddr> = vec![];
        for response in rx {
            active_ips.push(response)
        }

        println!("Done");

        ActiveIps { active_ips }
    }

    fn generate_ip_range(start_ip: IpAddr, sweep_octat: u8) -> Vec<IpAddr> {
        let mut res_ips: Vec<IpAddr> = vec![];

        for octat_val in 0..=255 {
            res_ips.push(form_ip_with_octat_replace(
                &start_ip,
                sweep_octat,
                octat_val,
            ));
        }
        res_ips
    }
}

fn form_ip_with_octat_replace(ip: &IpAddr, sweep_octat: u8, octat_val: u8) -> IpAddr {
    match ip {
        IpAddr::V4(ipv4) => {
            let mut octets = ipv4.octets();
            // sweep_octat is 0-3, representing the position to replace
            if sweep_octat < 4 {
                octets[sweep_octat as usize] = octat_val;
            }
            IpAddr::V4(Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]))
        }
        IpAddr::V6(_) => *ip, // Return original IP if IPv6
    }
}

fn is_port_open(ip: IpAddr, port: i32, tx: mpsc::Sender<i32>) {
    // println!("Scanning {ip}:{port}");
    if TcpStream::connect(format!("{}:{}", ip, port)).is_ok() {
        tx.send(port).unwrap();
    }
}

fn scan_for_ip(ip: IpAddr, tx: mpsc::Sender<IpAddr>) {
    if TcpStream::connect(format!("{}:{}", ip, 80)).is_ok()
        || TcpStream::connect(format!("{}:{}", ip, 443)).is_ok()
    {
        tx.send(ip).unwrap()
    }
}
