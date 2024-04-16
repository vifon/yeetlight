use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::io::BufReader;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

const PORT: u16 = 55443;
const IP_ADDR: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
const SOCKET_ADDR: SocketAddr = SocketAddr::new(IP_ADDR, PORT);

use yeetlight::command::Command;
use yeetlight::Response;

pub struct BulbConnection {
    connection: TcpStream,
}

impl BulbConnection {
    pub async fn receive(&mut self) -> io::Result<String> {
        self.receive_and_respond(Response::default()).await
    }
    pub async fn receive_and_respond(&mut self, mut response: Response) -> io::Result<String> {
        let mut reader = BufReader::new(&mut self.connection);
        let mut message = String::new();
        reader.read_line(&mut message).await?;
        let message = message.trim_end();

        match serde_json::from_str::<Command>(message) {
            Ok(command) => {
                // Assume this is the message for us.
                response.id = command.id;

                let payload = serde_json::to_string(&response)?;
                let payload = payload + "\r\n";

                self.connection.write_all(payload.as_bytes()).await?;

                // Always return with id:1 for predictable test outputs.
                let message = serde_json::to_string(&Command { id: 1, ..command })?;
                Ok(message.to_owned())
            }
            // Not a Command?  That's okay, let the caller inspect the contents.
            Err(_) => Ok(message.to_owned()),
        }
    }
}

pub struct BulbListener {
    listener: TcpListener,
    pub addr: SocketAddr,
}

impl BulbListener {
    pub async fn serve() -> io::Result<Self> {
        let socket = SOCKET_ADDR;
        let listener = tokio::net::TcpListener::bind(socket).await?;
        Ok(BulbListener {
            listener,
            addr: socket,
        })
    }

    pub async fn accept(&self) -> io::Result<BulbConnection> {
        let (connection, _) = self.listener.accept().await?;
        Ok(BulbConnection { connection })
    }
}
