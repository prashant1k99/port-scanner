use std::net::{IpAddr, Ipv4Addr};

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct Arguments {
    pub host: IpAddr,
    pub flags: Option<Vec<Flags>>,
    is_sweep: bool,
}

#[derive(Debug, PartialEq)]
pub struct Flags {
    pub name: String,
    pub value: Vec<String>,
}

impl Arguments {
    pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Insufficient Arguments passed");
        }

        let help_flags = ["--help", "-h"];
        if let Some(arg) = args.get(1) {
            if help_flags.contains(&arg.as_str()) {
                return Err("Help flag passed");
            }
        }

        if !IpAddr::V4(args[1].parse().unwrap()).is_ipv4()
            && !IpAddr::V6(args[1].parse().unwrap()).is_ipv6()
        {
            println!("This is neither IPv4 or Ipv6")
        }

        Ok(Arguments {
            host: IpAddr::V4(Ipv4Addr::new(127, 1, 1, 1)),
            flags: None,
            is_sweep: false,
        })
    }
}
