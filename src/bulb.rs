use log::info;
use std::io;
use std::net::{AddrParseError, IpAddr, SocketAddr};
use std::str::FromStr;
use tokio::net::TcpStream;

use crate::BulbConnection;

const PORT: u16 = 55443;

#[derive(Debug)]
pub struct Bulb {
    addr: SocketAddr,
}

impl FromStr for Bulb {
    type Err = AddrParseError;

    fn from_str(addr: &str) -> Result<Self, Self::Err> {
        Ok(Bulb::new(addr.parse()?))
    }
}

impl Bulb {
    pub fn new(addr: IpAddr) -> Self {
        Bulb {
            addr: SocketAddr::new(addr, PORT),
        }
    }

    pub async fn connect(&self) -> io::Result<BulbConnection> {
        info!("Connecting to: {}", self.addr);
        let connection = BulbConnection {
            stream: TcpStream::connect(&self.addr).await?,
        };
        info!("Connected to: {}", self.addr);
        Ok(connection)
    }
}
