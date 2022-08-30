mod arguments;

use std::env;
use std::sync::mpsc;
use std::thread;
use std::net::{IpAddr, TcpStream};
use std::io::Write;
use crate::arguments::Arguments;

const MAX_PORT: u16 = 65535;

fn main() {
    let args: Vec<String> = env::args().collect(); // collect args
    let args = Arguments::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {
            std::process::exit(0);
        } else {
            eprintln!("Problem parsing arguments: {}", err);
            std::process::exit(1);
        }
    });

    let (tx, rx) = mpsc::channel();

    for i in 0..args.threads {
        let tx = tx.clone();
        thread::spawn(move || {
            scan(tx, i, args.ip, args.threads);
        });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }

    println!("---------");

    out.sort();
    for p in out {
        println!("{} is open", p);
    }
}

fn scan(tx: mpsc::Sender<u16>,
        thread_id: u16,
        ip: IpAddr,
        threads_num: u16) {
    let mut port = thread_id + 1;

    loop {
        match TcpStream::connect((ip, port)) {
            Ok(_) => {
                print!(".");
                std::io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            },
            Err(_) => {},
        }

        if (MAX_PORT - port) <= threads_num {
            break;
        }

        port += threads_num;
    }
}