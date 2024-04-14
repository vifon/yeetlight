use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    id: u32,
    method: String,
    params: Value,
}

impl Command {
    pub fn new(method: &str, params: Value) -> Command {
        Command {
            id: 1,
            method: method.to_owned(),
            params,
        }
    }
}
