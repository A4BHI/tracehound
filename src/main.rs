use std::fs::File;
use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::path::Path;
use std::time::Duration;

fn load_servics() {
    let path = Path::new("/etc/services");
    let mut f = File::open(path).expect("Error reading file");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Cannot read");
    println!("{}", s);
}

fn main() -> Result<(), io::Error> {
    load_servics();
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
    let addr = SocketAddr::new(ip, 8080);
    match TcpStream::connect_timeout(&addr, Duration::new(2, 0)) {
        Ok(_con) => {
            println!("Port Open");
        }
        Err(err) => {
            println!("Port closed  : {}", err);
        }
    }
    Ok(())
}
