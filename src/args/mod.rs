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
                flags: handle_arg_flags(&args[2..])?,
                is_sweep: false,
            }),
            Err(_) => {
                if args[1].contains("*") {
                    // Split the arguments based on the . and try to replace the * with 1
                    if let Ok(val) = args[1].replace("*", "1").parse::<IpAddr>() {
                        return Ok(Arguments {
                            host: val,
                            flags: handle_arg_flags(&args[2..])?,
                            is_sweep: true,
                        });
                    }
                } else if args[1] == "localhost" {
                    return Ok(Arguments {
                        host: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                        flags: handle_arg_flags(&args[2..])?,
                        is_sweep: false,
                    });
                }

                Err("Invalid host name")
            }
        }
    }

    pub fn get_port(&self) -> Option<Vec<i32>> {
        if let Some(flags) = &self.flags {
            for flag in flags {
                if flag.name == "p" {
                    return Some(
                        flag.value
                            .iter()
                            .filter_map(|v| v.parse::<i32>().ok())
                            .collect(),
                    );
                }
            }
        }
        None
    }

    pub fn get_thread_counts(&self) -> Option<usize> {
        if let Some(flags) = &self.flags {
            for flag in flags {
                if flag.name == "j" {
                    return flag.value.first()?.parse::<usize>().ok();
                }
            }
        }
        None
    }

    pub fn is_sweep_scan(&self) -> bool {
        self.is_sweep
    }
}

const SUPPORTED_FLAGS: [&str; 2] = ["-j", "-p"];

fn handle_arg_flags(args: &[String]) -> Result<Option<Vec<Flags>>, &'static str> {
    let mut result_flags = vec![];
    for arg in args {
        let val: Vec<_> = arg.split("=").collect();
        if SUPPORTED_FLAGS.contains(&val[0]) {
            result_flags.push(format_arg(val)?);
        }
    }
    if !result_flags.is_empty() {
        Ok(Some(result_flags))
    } else {
        Ok(None)
    }
}

fn format_arg(vals: Vec<&str>) -> Result<Flags, &'static str> {
    let flag_name = vals[0];
    let flag_value: Vec<String> = vals[1].split(",").map(|val| val.to_string()).collect();
    if flag_name == "-p" {
        for port in &flag_value {
            match port.parse::<i32>() {
                Ok(val) if (1..=65535).contains(&val) => val,
                _ => return Err("Invalid port argument"),
            };
        }
    }
    Ok(Flags {
        name: flag_name.replace("-", ""),
        value: flag_value,
    })
}
