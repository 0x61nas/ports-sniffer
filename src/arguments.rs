use std::net::IpAddr;
use std::str::FromStr;
use colored::Colorize;

pub struct Arguments {
    pub flag: String,
    pub ip: IpAddr,
    pub threads: u16,
}

impl Arguments {
    pub fn new(args: &[String]) -> Result<Arguments, String> {
        if args.len() > 4 {
            return Err("too many arguments".to_string());
        }

        let ip =
            if !args.len() < 2 || (args.len() >= 2 && !args[1].starts_with('-')) {
                match IpAddr::from_str(&args[1].clone()) {
                    Ok(ip) => ip,
                    Err(e) => return Err(format!(
                        "invalid ip address, you can not type any ip if you want use local ip: {e}"
                    )),
                }
            } else {
                match local_ip_address::local_ip() {
                    Ok(ip) => ip,
                    Err(e) => return Err(format!("Can't get local ip address: {e}")),
                }
            };

        if args.len() > 1 && args[1].starts_with('-') {
            let flag = args[1].clone();
            if flag == "-h" || flag == "--help" {
                println!("{}",
                    "Usage: ports-sniffer [ip] [-t <threads>]\n\
                    -h, --help\t\tShow this help message\n\
                    -v, --version\t\tShow version\n\
                    -t, --threads\t\tSet the number of threads to use\n"
                        .green()
                );
                return Err("info".to_string());
            } else if flag == "-v" || flag == "--version" {
                println!("ports-sniffer version 0.1.0");
                return Err("info".to_string());
            } else {
                return Err("invalid flag".to_string());
            }
        }

        Ok(Arguments {
            flag: String::new(),
            ip,
            threads: {
                if args.len() >= 2 {
                    if args[2] == "-t" || args[2] == "--threads" {
                        match args[3].parse::<u16>() {
                            Ok(num) => num,
                            Err(e) => return Err(format!("invalid number of threads: {e}")),
                        }
                    } else {
                        return Err("invalid argument".to_string());
                    }
                } else {
                    4
                }
            },
        })
    }
}
