use log::info;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::io;
use std::io::{prelude::*, BufReader};
use std::net::TcpStream;
use std::num::ParseIntError;
use thiserror::Error;

use crate::command::Command;

pub struct Bulb {
    addr: String,
}

#[derive(Copy, Clone)]
pub enum Effect {
    Smooth(u16),
    Sudden,
}

impl Effect {
    pub fn effect(&self) -> &'static str {
        match self {
            Effect::Smooth(_) => "smooth",
            Effect::Sudden => "sudden",
        }
    }

    pub fn duration(&self) -> u16 {
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
    #[error("Percentage {} not within [{}..{}]", .0, Percentage::MIN, Percentage::MAX)]
    Percentage(i16),
}

trait BoundedRange {
    const MIN: i32;
    const MAX: i32;

    fn valid_range(value: i32) -> bool {
        (Self::MIN..=Self::MAX).contains(&value)
    }
}

#[derive(Copy, Clone)]
pub struct Brightness(u16);
impl BoundedRange for Brightness {
    const MIN: i32 = 1;
    const MAX: i32 = 100;
}
impl Brightness {
    pub fn new(brightness: u16) -> Result<Self, RangeError> {
        if Self::valid_range(brightness as i32) {
            Ok(Brightness(brightness))
        } else {
            Err(RangeError::Brightness(brightness))
        }
    }
}

#[derive(Copy, Clone)]
pub struct Temperature(u16);
impl BoundedRange for Temperature {
    const MIN: i32 = 1700;
    const MAX: i32 = 6500;
}
impl Temperature {
    pub fn new(temperature: u16) -> Result<Self, RangeError> {
        if Self::valid_range(temperature as i32) {
            Ok(Temperature(temperature))
        } else {
            Err(RangeError::Temperature(temperature))
        }
    }
}

#[derive(Copy, Clone)]
pub struct Color(u32);
impl Color {
    pub fn from_hex(hex: &str) -> Result<Color, ParseIntError> {
        Ok(Color(u32::from_str_radix(hex, 16)?))
    }
}

#[derive(Copy, Clone)]
pub struct Percentage(i16);
impl BoundedRange for Percentage {
    const MIN: i32 = -100;
    const MAX: i32 = 100;
}
impl Percentage {
    pub fn new(percentage: i16) -> Result<Self, RangeError> {
        if Self::valid_range(percentage as i32) {
            Ok(Percentage(percentage))
        } else {
            Err(RangeError::Percentage(percentage))
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

    pub fn set_brightness(
        &self,
        Brightness(brightness): Brightness,
        effect: Effect,
    ) -> io::Result<Value> {
        self.call(Command::new(
            "set_bright",
            json![[brightness, effect.effect(), effect.duration()]],
        ))
    }

    pub fn adjust_brightness(
        &self,
        Percentage(percentage): Percentage,
        duration: u16,
    ) -> io::Result<Value> {
        self.call(Command::new("adjust_bright", json![[percentage, duration]]))
    }

    pub fn set_temperature(
        &self,
        Temperature(temperature): Temperature,
        effect: Effect,
    ) -> io::Result<Value> {
        self.call(Command::new(
            "set_ct_abx",
            json![[temperature, effect.effect(), effect.duration()]],
        ))
    }

    pub fn set_color(&self, Color(color): Color, effect: Effect) -> io::Result<Value> {
        self.call(Command::new(
            "set_rgb",
            json![[color, effect.effect(), effect.duration()]],
        ))
    }

    pub fn get_props(&self, props: &[&str]) -> io::Result<Vec<String>> {
        let response = self.call(Command::new("get_prop", json!(props)))?;
        let values: Vec<String> = response
            .as_object()
            .expect("Got a response but not an object")["result"]
            .as_array()
            .expect("No results in the response")
            .iter()
            .map(|x| x.as_str().expect("Got an invalid prop value").to_owned())
            .collect();
        Ok(values)
    }

    pub fn get_props_map<'a>(&self, props: &[&'a str]) -> io::Result<BTreeMap<&'a str, String>> {
        let values = self.get_props(props)?;
        Ok(BTreeMap::from_iter(props.iter().copied().zip(values)))
    }
}
