use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::io::BufReader;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

const PORT: u16 = 55443;
const IP_ADDR: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
const SOCKET_ADDR: SocketAddr = SocketAddr::new(IP_ADDR, PORT);

use yeetlight::Response;

pub struct BulbConnection {
    connection: TcpStream,
    last_command_id: u16,
}

impl BulbConnection {
    fn new(connection: TcpStream) -> io::Result<Self> {
        Ok(Self {
            connection,
            last_command_id: 0,
        })
    }

    pub async fn receive(&mut self) -> io::Result<String> {
        self.receive_and_respond(Response::default()).await
    }

    pub async fn receive_and_respond(&mut self, response: Response) -> io::Result<String> {
        let mut reader = BufReader::new(&mut self.connection);
        let mut message = String::new();
        reader.read_line(&mut message).await?;
        let message = message.trim_end();

        self.last_command_id += 1;
        let response = Response {
            id: self.last_command_id,
            ..response
        };

        let payload = serde_json::to_string(&response)?;
        let payload = payload + "\r\n";

        self.connection.write_all(payload.as_bytes()).await?;

        Ok(message.to_owned())
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
        BulbConnection::new(connection)
    }
}
