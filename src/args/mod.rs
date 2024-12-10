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

const HELP_FLAGS: [&str; 2] = ["--help", "-h"];

impl Arguments {
    pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Insufficient Arguments passed");
        }

        if let Some(arg) = args.get(1) {
            if HELP_FLAGS.contains(&arg.as_str()) {
                return Err("Help flag passed");
            }
        }

        match args[1].parse() {
            Ok(host) => Ok(Arguments {
                host,
                flags: handle_arg_flags(&args[2..]),
                is_sweep: false,
            }),
            Err(_) => {
                if args[1].contains("*") {
                    // Split the arguments based on the . and try to replace the * with 1
                    if let Ok(val) = args[1].replace("*", "1").parse::<IpAddr>() {
                        return Ok(Arguments {
                            host: val,
                            flags: handle_arg_flags(&args[2..]),
                            is_sweep: true,
                        });
                    }
                }
                Err("Invalid host name")
            }
        }
    }
}

const SUPPORTED_FLAGS: [&str; 2] = ["-j", "-p"];

fn handle_arg_flags(args: &[String]) -> Option<Vec<Flags>> {
    let mut result_flags = vec![];
    for arg in args {
        let val: Vec<_> = arg.split("=").collect();
        if SUPPORTED_FLAGS.contains(&val[0]) {
            result_flags.push(format_arg(val));
        }
    }
    if !result_flags.is_empty() {
        Some(result_flags)
    } else {
        None
    }
}

fn format_arg(vals: Vec<&str>) -> Flags {
    let flag_name = vals[0];
    let flag_value: Vec<String> = vals[1].split(",").map(|val| val.to_string()).collect();
    Flags {
        name: flag_name.replace("-", ""),
        value: flag_value,
    }
}
