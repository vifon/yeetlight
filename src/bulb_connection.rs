use log::{info, warn};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::io;
use tokio::io::BufReader;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::command::Command;
use crate::params::{Brightness, Color, Effect, Percentage, Temperature};

#[derive(Debug)]
pub struct BulbConnection {
    pub(crate) stream: TcpStream,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Response {
    pub id: u16,
    pub result: Option<Value>,
    pub error: Option<Value>,
}

impl BulbConnection {
    async fn call(&mut self, command: Command) -> io::Result<Response> {
        let payload = serde_json::to_string(&command)?;
        // Additional space to align the output with "Received".
        info!("Sending:  {}", payload);
        let payload = payload + "\r\n";
        self.stream.write_all(payload.as_bytes()).await?;

        loop {
            let mut response = String::new();
            let mut reader = BufReader::new(&mut self.stream);
            reader.read_line(&mut response).await?;
            let response = response.trim_end();
            info!("Received: {}", response);

            match serde_json::from_str::<Response>(response) {
                Ok(response) => {
                    info!("Parsed as: {:?}", response);
                    if response.id == command.id {
                        return Ok(response);
                    } else {
                        warn!("Not matching id, ignoring: {}", response.id);
                    }
                }
                Err(err) => {
                    warn!("Unable to parse, ignoring: {}", err);
                }
            }
        }
    }

    pub async fn set_power(&mut self, state: bool, effect: Effect) -> io::Result<Response> {
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
        &mut self,
        Brightness(brightness): Brightness,
        effect: Effect,
    ) -> io::Result<Response> {
        self.call(Command::new(
            "set_bright",
            json![[brightness, effect.effect(), effect.duration()]],
        ))
        .await
    }

    pub async fn adjust_brightness(
        &mut self,
        Percentage(percentage): Percentage,
        duration: u16,
    ) -> io::Result<Response> {
        self.call(Command::new("adjust_bright", json![[percentage, duration]]))
            .await
    }

    pub async fn set_temperature(
        &mut self,
        Temperature(temperature): Temperature,
        effect: Effect,
    ) -> io::Result<Response> {
        self.call(Command::new(
            "set_ct_abx",
            json![[temperature, effect.effect(), effect.duration()]],
        ))
        .await
    }

    pub async fn set_color(&mut self, Color(color): Color, effect: Effect) -> io::Result<Response> {
        self.call(Command::new(
            "set_rgb",
            json![[color, effect.effect(), effect.duration()]],
        ))
        .await
    }

    pub async fn get_props(&mut self, props: &[&str]) -> io::Result<Vec<String>> {
        let response = self.call(Command::new("get_prop", json!(props))).await?;
        let values: Vec<String> = response
            .result
            .expect("No results in the response")
            .as_array()
            .expect("Results are not an array")
            .iter()
            .map(|x| x.as_str().expect("Got an invalid prop value").to_owned())
            .collect();
        Ok(values)
    }

    pub async fn get_props_map<'a>(
        &mut self,
        props: &[&'a str],
    ) -> io::Result<BTreeMap<&'a str, String>> {
        let values = self.get_props(props).await?;
        Ok(BTreeMap::from_iter(props.iter().copied().zip(values)))
    }
}
