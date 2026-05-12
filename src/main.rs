use std::env;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::time::{Duration, Instant};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("usage: portcheck <host> <port> [port2] ...");
        eprintln!("       portcheck google.com 80 443");
        eprintln!("       portcheck 192.168.1.1 22 80 443");
        std::process::exit(1);
    }

    let host = &args[1];
    let timeout = Duration::from_millis(3000);

    // find --timeout flag if present
    let timeout = {
        let mut t = timeout;
        let mut i = 2;
        while i < args.len() {
            if args[i] == "--timeout" && i + 1 < args.len() {
                if let Ok(ms) = args[i + 1].parse::<u64>() {
                    t = Duration::from_millis(ms);
                }
            }
            i += 1;
        }
        t
    };

    for port_str in &args[2..] {
        if port_str == "--timeout" {
            break;
        }
        let port: u16 = match port_str.parse() {
            Ok(p) => p,
            Err(_) => {
                eprintln!("skipping invalid port: {}", port_str);
                continue;
            }
        };

        let addr_str = format!("{}:{}", host, port);
        let addr: SocketAddr = match addr_str.to_socket_addrs() {
            Ok(mut addrs) => match addrs.next() {
                Some(a) => a,
                None => {
                    println!("{:<32} error (could not resolve)", addr_str);
                    continue;
                }
            },
            Err(_) => {
                println!("{:<32} error (could not resolve)", addr_str);
                continue;
            }
        };

        let start = Instant::now();
        match TcpStream::connect_timeout(&addr, timeout) {
            Ok(_) => {
                let ms = start.elapsed().as_millis();
                println!("{:<32} open    ({}ms)", addr_str, ms);
            }
            Err(_) => {
                println!("{:<32} closed", addr_str);
            }
        }
    }
}
