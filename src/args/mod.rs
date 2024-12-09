use std::net::{IpAddr, Ipv4Addr};

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct Arguments {
    pub host: IpAddr,
    pub flags: Option<Vec<Flags>>,
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
        Ok(Arguments {
            host: IpAddr::V4(Ipv4Addr::new(127, 1, 1, 1)),
            flags: None,
        })
    }
}
