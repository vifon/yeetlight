use log::info;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::io;
use std::io::{prelude::*, BufReader};
use std::net::TcpStream;
use std::num::ParseIntError;
use thiserror::Error;

use super::command::Command;

pub struct Bulb {
    addr: String,
}

pub enum Effect {
    Smooth(u32),
    Sudden,
}

impl Effect {
    pub fn effect(&self) -> &'static str {
        match self {
            Effect::Smooth(_) => "smooth",
            Effect::Sudden => "sudden",
        }
    }

    pub fn duration(&self) -> u32 {
        match self {
            Effect::Smooth(x) => *x,
            Effect::Sudden => 0,
        }
    }
}

#[derive(Error, Debug)]
pub enum RangeError {
    #[error("Temperature {} not within [{}..{}]", .0, Temperature::MIN, Temperature::MAX)]
    Temperature(u16),
    #[error("Brightness {} not within [{}..{}]", .0, Brightness::MIN, Brightness::MAX)]
    Brightness(u16),
}

trait BoundedRange {
    const MIN: u16;
    const MAX: u16;

    fn valid_range(value: u16) -> bool {
        (Self::MIN..=Self::MAX).contains(&value)
    }
}

pub struct Brightness(u16);
impl BoundedRange for Brightness {
    const MIN: u16 = 1;
    const MAX: u16 = 100;
}
impl Brightness {
    pub fn new(brightness: u16) -> Result<Self, RangeError> {
        if Self::valid_range(brightness) {
            Ok(Brightness(brightness))
        } else {
            Err(RangeError::Brightness(brightness))
        }
    }
}

pub struct Temperature(u16);
impl BoundedRange for Temperature {
    const MIN: u16 = 1700;
    const MAX: u16 = 6500;
}
impl Temperature {
    pub fn new(temperature: u16) -> Result<Self, RangeError> {
        if Self::valid_range(temperature) {
            Ok(Temperature(temperature))
        } else {
            Err(RangeError::Temperature(temperature))
        }
    }
}

pub struct Color(u32);
impl Color {
    pub fn from_hex(hex: &str) -> Result<Color, ParseIntError> {
        Ok(Color(u32::from_str_radix(hex, 16)?))
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

    pub fn set_power(&self, state: bool, effect: Effect) -> io::Result<Value> {
        let state = match state {
            true => "on",
            false => "off",
        };

        self.call(Command::new(
            "set_power",
            json![[state, effect.effect(), effect.duration()]],
        ))
    }

    pub fn set_brightness(&self, brightness: Brightness, effect: Effect) -> io::Result<Value> {
        self.call(Command::new(
            "set_bright",
            json![[brightness.0, effect.effect(), effect.duration()]],
        ))
    }

    pub fn set_temperature(&self, temperature: Temperature, effect: Effect) -> io::Result<Value> {
        self.call(Command::new(
            "set_ct_abx",
            json![[temperature.0, effect.effect(), effect.duration()]],
        ))
    }

    pub fn set_color(&self, color: Color, effect: Effect) -> io::Result<Value> {
        self.call(Command::new(
            "set_rgb",
            json![[color.0, effect.effect(), effect.duration()]],
        ))
    }

    pub fn get_props<'a>(&self, props: &[&'a str]) -> io::Result<BTreeMap<&'a str, String>> {
        let response = self.call(Command::new("get_prop", json!(props)))?;
        let values: Vec<String> = response
            .as_object()
            .expect("Got a response but not an object")["result"]
            .as_array()
            .expect("No results in the response")
            .into_iter()
            .map(|x| x.as_str().expect("Got an invalid prop value").to_owned())
            .collect();
        Ok(BTreeMap::from_iter(
            props.into_iter().map(|x| *x).zip(values),
        ))
    }
}
