use log::info;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io;
use std::io::{prelude::*, BufReader};
use std::net::TcpStream;

#[derive(Serialize, Deserialize)]
struct Command {
    id: u32,
    method: String,
    params: Value,
}

struct Bulb {
    addr: String,
}

impl Bulb {
    fn new(addr: &str) -> Self {
        Bulb {
            addr: String::from(addr) + ":55443",
        }
    }

    fn connect(self: &Self) -> io::Result<TcpStream> {
        TcpStream::connect(&self.addr)
    }

    fn call(self: &Self, command: &Command) -> io::Result<()> {
        let mut stream = self.connect()?;

        let payload = serde_json::to_string(command)?;
        info!("Sending: {}", payload);
        let payload = payload + "\r\n";
        stream.write(payload.as_bytes())?;

        let mut response = String::new();
        let mut reader = BufReader::new(stream);
        reader.read_line(&mut response)?;

        info!("Received: {}", response.trim_end());

        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    simple_logger::init().unwrap();

    let b = Bulb::new("192.168.2.162");
    let c = Command {
        id: 1,
        method: String::from("set_power"),
        params: json![["on", "smooth", 500]],
    };

    b.call(&c)?;

    Ok(())
}
