use std::fs::File;
use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::path::Path;
use std::time::Duration;

fn load_servics(ip: std::net::IpAddr) {
    let path = Path::new("/etc/services");
    let mut f = File::open(path).expect("Error reading file");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Cannot read");
    for lines in s.lines() {
        let line = lines.split('#').next().unwrap_or("").trim();
        // println!("{}", line);
        // println!("+++++++++++++++++++++++++++++");
        if line.is_empty() {
            continue;
        }

        let Some(service_entry) = line.split_whitespace().nth(1) else {
            continue;
        };

        let Some((port, protocol)) = service_entry.split_once('/') else {
            continue;
        };

        if protocol != "tcp" {
            continue;
        }

        if let Ok(port) = port.parse::<u16>() {
            let addr = SocketAddr::new(ip, port);
            match TcpStream::connect_timeout(&addr, Duration::new(2, 0)) {
                Ok(_con) => {
                    println!("Open => {}/{}", port, protocol);
                }
                Err(err) => {
                    println!("Closed => {}/{} : {}", port, protocol, err);
                }
            }
        }
    }
    // println!("{}", s);
}

fn main() -> Result<(), io::Error> {
    let mut rawip = String::new();
    println!("Welcome to ruscan!");
    io::stdout()
        .write_all("Input ip adress:".as_bytes())
        .expect("Invalid INput");
    io::stdout().flush()?;
    io::stdin().read_line(&mut rawip)?;
    let ip = rawip
        .trim()
        .parse::<std::net::IpAddr>()
        .expect("Error parsing");
    load_servics(ip);
    Ok(())
}
