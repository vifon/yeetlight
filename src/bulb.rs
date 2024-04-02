use log::info;
use serde_json::{json, Value};
use std::io;
use std::io::{prelude::*, BufReader};
use std::net::TcpStream;

use super::command::Command;

pub struct Bulb {
    addr: String,
}

#[allow(dead_code)]
pub enum BulbEffect {
    Smooth(u32),
    Sudden,
}

impl BulbEffect {
    pub fn effect(&self) -> &'static str {
        match self {
            BulbEffect::Smooth(_) => "smooth",
            BulbEffect::Sudden => "sudden",
        }
    }

    pub fn duration(&self) -> u32 {
        match self {
            BulbEffect::Smooth(x) => *x,
            BulbEffect::Sudden => 0,
        }
    }
}

impl Bulb {
    pub fn new(addr: &str) -> Self {
        Bulb {
            addr: String::from(addr) + ":55443",
        }
    }

    fn connect(&self) -> io::Result<TcpStream> {
        TcpStream::connect(&self.addr)
    }

    fn call(&self, command: Command) -> io::Result<Value> {
        let mut stream = self.connect()?;

        let payload = serde_json::to_string(&command)?;
        info!("Sending: {}", payload);
        let payload = payload + "\r\n";
        stream.write_all(payload.as_bytes())?;

        let mut response = String::new();
        let mut reader = BufReader::new(stream);
        reader.read_line(&mut response)?;
        let response = response.trim_end();

        info!("Received: {}", response);
        let response = serde_json::from_str(response)?;

        Ok(response)
    }

    pub fn set_power(&self, state: bool, effect: BulbEffect) -> io::Result<Value> {
        let state = match state {
            true => "on",
            false => "off",
        };

        self.call(Command::new(
            "set_power",
            json![[state, effect.effect(), effect.duration()]],
        ))
    }
}
