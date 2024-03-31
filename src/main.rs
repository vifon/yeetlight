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

impl Command {
    fn new(method: &str, params: Value) -> Command {
        Command {
            id: 1,
            method: method.to_owned(),
            params,
        }
    }
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

    fn call(self: &Self, command: &Command) -> io::Result<Value> {
        let mut stream = self.connect()?;

        let payload = serde_json::to_string(command)?;
        info!("Sending: {}", payload);
        let payload = payload + "\r\n";
        stream.write(payload.as_bytes())?;

        let mut response = String::new();
        let mut reader = BufReader::new(stream);
        reader.read_line(&mut response)?;
        let response = response.trim_end();

        info!("Received: {}", response);
        let response = serde_json::from_str(response)?;

        Ok(response)
    }
}

fn main() -> std::io::Result<()> {
    simple_logger::init().unwrap();

    let b = Bulb::new("192.168.2.162");
    let c = Command::new("set_power", json![["on", "smooth", 500]]);

    b.call(&c)?;

    Ok(())
}
