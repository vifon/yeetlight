use log::{info, warn};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::BTreeMap;
use std::io;
use tokio::io::BufReader;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::params::{Brightness, Color, Effect, Percentage, Temperature};

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    pub id: u16,
    pub method: String,
    pub params: Vec<Value>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Response {
    pub id: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Map<String, Value>>,
}

#[derive(Debug)]
pub struct BulbConnection {
    stream: TcpStream,
    last_command_id: u16,
}

impl BulbConnection {
    pub fn new(stream: TcpStream) -> io::Result<Self> {
        Ok(Self {
            stream,
            last_command_id: 0,
        })
    }

    fn new_command(&mut self, method: &str, params: Vec<Value>) -> Command {
        self.last_command_id += 1;
        Command {
            id: self.last_command_id,
            method: method.to_owned(),
            params,
        }
    }

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

        let command = self.new_command(
            "set_power",
            vec![
                state.into(),
                effect.effect().into(),
                effect.duration().into(),
            ],
        );

        self.call(command).await
    }

    pub async fn set_brightness(
        &mut self,
        Brightness(brightness): Brightness,
        effect: Effect,
    ) -> io::Result<Response> {
        let command = self.new_command(
            "set_bright",
            vec![
                brightness.into(),
                effect.effect().into(),
                effect.duration().into(),
            ],
        );
        self.call(command).await
    }

    pub async fn adjust_brightness(
        &mut self,
        Percentage(percentage): Percentage,
        duration: u16,
    ) -> io::Result<Response> {
        let command = self.new_command("adjust_bright", vec![percentage.into(), duration.into()]);
        self.call(command).await
    }

    pub async fn set_temperature(
        &mut self,
        Temperature(temperature): Temperature,
        effect: Effect,
    ) -> io::Result<Response> {
        let command = self.new_command(
            "set_ct_abx",
            vec![
                temperature.into(),
                effect.effect().into(),
                effect.duration().into(),
            ],
        );
        self.call(command).await
    }

    pub async fn set_color(&mut self, Color(color): Color, effect: Effect) -> io::Result<Response> {
        let command = self.new_command(
            "set_rgb",
            vec![
                color.into(),
                effect.effect().into(),
                effect.duration().into(),
            ],
        );
        self.call(command).await
    }

    pub async fn get_props(&mut self, props: &[&str]) -> io::Result<Vec<String>> {
        let props = props.iter().map(|x| Value::from(*x)).collect();
        let command = self.new_command("get_prop", props);
        let response = self.call(command).await?;
        let values: Vec<String> = response
            .result
            .expect("No results in the response")
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
