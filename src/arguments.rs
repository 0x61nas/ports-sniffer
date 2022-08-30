use std::net::IpAddr;
use std::str::FromStr;

pub struct Arguments {
    pub flag: String,
    pub ip: IpAddr,
    pub threads: u16,
}

impl Arguments {
    pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments")
        } else if args.len() > 4 {
            return Err("too many arguments")
        }

        let ip = args[1].clone();
        return if let Ok(ip) = IpAddr::from_str(&ip) {
            Ok(Arguments {
                flag: String::new(),
                ip,
                threads: {
                    if args.len() == 4 {
                        if args[2] == "-t" || args[2] == "--threads" {
                            args[3].parse::<u16>().unwrap()
                        } else {
                            return Err("invalid argument")
                        }
                    } else {
                        4
                    }
                }
            })
        } else {
            let flag = args[1].clone();
            if flag == "-h" || flag == "--help" {
                println!("Usage: ports-sniffer <ip> [-t <threads>]\n\
                    -h, --help\t\tShow this help message\n\
                    -v, --version\t\tShow version\n\
                    -t, --threads\t\tSet the number of threads to use\n");
                Err("info")
            }else if flag == "-v" || flag == "--version" {
                println!("ports-sniffer version 0.0.2");
                Err("info")
            } else {
                Err("invalid argument")
            }
        }
    }
}