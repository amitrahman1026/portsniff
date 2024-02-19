use std::env;
use std::io::{self, Write};
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::process;
use std::str::FromStr;
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Duration;

const MAX_PORT: u16 = 65535;

struct Arguments {
    ipaddr: IpAddr,
    threads: u16,
}

fn parse_arguments(args: &[String]) -> Result<Arguments, &'static str> {
    match args.len() {
        2 => match IpAddr::from_str(&args[1]) {
            Ok(ipaddr) => Ok(Arguments { ipaddr, threads: 4 }),
            Err(_) => Err("Invalid IP address"),
        },
        4 if args[1] == "-t" => match args[2].parse::<u16>() {
            Ok(threads) => match IpAddr::from_str(&args[3]) {
                Ok(ipaddr) => Ok(Arguments { ipaddr, threads }),
                Err(_) => Err("Invalid IP address"),
            },
            Err(_) => Err("Failed to parse number of threads"),
        },
        _ => Err("Usage: <ipaddr> or -t <threads> <ipaddr>"),
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port = start_port;
    // 1 Second timeout because I'm impatient
    let timeout = Duration::new(1, 0);
    loop {
        if MAX_PORT - port <= num_threads {
            break;
        }
        let socket = SocketAddr::new(addr, port);
        if TcpStream::connect_timeout(&socket, timeout).is_ok() {
            print!(".");
            io::stdout().flush().unwrap();
            tx.send(port).unwrap();
        }
        port += num_threads;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let arguments = parse_arguments(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    let (tx, rx) = mpsc::channel();
    let num_threads = arguments.threads;

    for i in 0..num_threads {
        let tx = tx.clone();
        let addr = arguments.ipaddr;
        thread::spawn(move || scan(tx, i, addr, num_threads));
    }
    // Drop the original sender to close the channel
    drop(tx);
    let mut open_ports: Vec<u16> = rx.into_iter().collect();
    open_ports.sort();

    println!("");
    for port in open_ports {
        println!("Port {} is open", port);
    }
}
