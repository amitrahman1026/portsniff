use std::io::{self, Write};
use std::net::{IpAddr, SocketAddr};
use std::process;
use std::str::FromStr;
use std::{env, usize};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::time::{self, Duration};

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

async fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port = start_port;
    let timeout = Duration::from_secs(5);
    loop {
        if MAX_PORT - port <= num_threads {
            break;
        }
        let socket = SocketAddr::new(addr, port);
        let result = time::timeout(timeout, TcpStream::connect(socket)).await;
        if result.is_ok() {
            if let Ok(_) = result.unwrap() {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).await.unwrap();
            }
        }
        port += num_threads;
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let arguments = parse_arguments(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    let num_threads = arguments.threads;
    let (tx, mut rx) = mpsc::channel(num_threads as usize);

    for i in 0..num_threads {
        let tx = tx.clone();
        let addr = arguments.ipaddr;
        tokio::spawn(async move {
            scan(tx, i, addr, num_threads).await;
        });
    }
    // Drop the original sender to close the channel
    drop(tx);
    let mut open_ports = Vec::new();
    while let Some(port) = rx.recv().await {
        open_ports.push(port);
    }

    open_ports.sort();

    println!("");
    for port in open_ports {
        println!("Port {} is open", port);
    }
}
