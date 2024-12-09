use std::net::IpAddr;

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

        match args[1].parse() {
            Ok(host) => Ok(Arguments {
                host,
                flags: None,
                is_sweep: false,
            }),
            Err(_) => {
                if args[1].contains("*") {
                    // Split the arguments based on the . and try to replace the * with 1
                    if let Ok(val) = args[1].replace("*", "1").parse::<IpAddr>() {
                        return Ok(Arguments {
                            host: val,
                            flags: None,
                            is_sweep: true,
                        });
                    }
                }
                Err("Invalid host name")
            }
        }
    }
}
