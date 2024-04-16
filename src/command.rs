use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    pub id: u16,
    pub method: String,
    pub params: Value,
}

impl Command {
    pub fn new(method: &str, params: Value) -> Command {
        Command {
            id: rand::thread_rng().gen(),
            method: method.to_owned(),
            params,
        }
    }
}
