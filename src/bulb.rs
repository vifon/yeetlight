use log::info;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::io;
use std::net::{AddrParseError, IpAddr, SocketAddr};
use tokio::io::BufReader;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::command::Command;
use crate::params::{Brightness, Color, Effect, Percentage, Temperature};

const PORT: u16 = 55443;

pub struct Bulb {
    addr: SocketAddr,
}

impl Bulb {
    pub fn new(addr: IpAddr) -> Self {
        Bulb {
            addr: SocketAddr::new(addr, PORT),
        }
    }

    pub fn from_str(addr: &str) -> Result<Self, AddrParseError> {
        Ok(Bulb {
            addr: SocketAddr::new(addr.parse()?, PORT),
        })
    }

    async fn connect(&self) -> io::Result<TcpStream> {
        TcpStream::connect(&self.addr).await
    }

    async fn call(&self, command: Command) -> io::Result<Value> {
        let mut stream = self.connect().await?;

        let payload = serde_json::to_string(&command)?;
        info!("Sending: {}", payload);
        let payload = payload + "\r\n";
        stream.write_all(payload.as_bytes()).await?;

        let mut response = String::new();
        let mut reader = BufReader::new(stream);
        reader.read_line(&mut response).await?;
        let response = response.trim_end();

        info!("Received: {}", response);
        let response = serde_json::from_str(response)?;

        Ok(response)
    }

    pub async fn set_power(&self, state: bool, effect: Effect) -> io::Result<Value> {
        let state = match state {
            true => "on",
            false => "off",
        };

        self.call(Command::new(
            "set_power",
            json![[state, effect.effect(), effect.duration()]],
        ))
        .await
    }

    pub async fn set_brightness(
        &self,
        Brightness(brightness): Brightness,
        effect: Effect,
    ) -> io::Result<Value> {
        self.call(Command::new(
            "set_bright",
            json![[brightness, effect.effect(), effect.duration()]],
        ))
        .await
    }

    pub async fn adjust_brightness(
        &self,
        Percentage(percentage): Percentage,
        duration: u16,
    ) -> io::Result<Value> {
        self.call(Command::new("adjust_bright", json![[percentage, duration]]))
            .await
    }

    pub async fn set_temperature(
        &self,
        Temperature(temperature): Temperature,
        effect: Effect,
    ) -> io::Result<Value> {
        self.call(Command::new(
            "set_ct_abx",
            json![[temperature, effect.effect(), effect.duration()]],
        ))
        .await
    }

    pub async fn set_color(&self, Color(color): Color, effect: Effect) -> io::Result<Value> {
        self.call(Command::new(
            "set_rgb",
            json![[color, effect.effect(), effect.duration()]],
        ))
        .await
    }

    pub async fn get_props(&self, props: &[&str]) -> io::Result<Vec<String>> {
        let response = self.call(Command::new("get_prop", json!(props))).await?;
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

    pub async fn get_props_map<'a>(
        &self,
        props: &[&'a str],
    ) -> io::Result<BTreeMap<&'a str, String>> {
        let values = self.get_props(props).await?;
        Ok(BTreeMap::from_iter(props.iter().copied().zip(values)))
    }
}
