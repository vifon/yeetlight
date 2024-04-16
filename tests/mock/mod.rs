use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::io::BufReader;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

const PORT: u16 = 55443;
const IP_ADDR: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
const SOCKET_ADDR: SocketAddr = SocketAddr::new(IP_ADDR, PORT);

pub struct BulbConnection {
    connection: TcpStream,
}

impl BulbConnection {
    pub async fn receive(&mut self) -> io::Result<String> {
        // An empty valid JSON.
        self.receive_and_respond("{}").await
    }
    pub async fn receive_and_respond(&mut self, response: &str) -> io::Result<String> {
        let mut reader = BufReader::new(&mut self.connection);
        let mut message = String::new();
        reader.read_line(&mut message).await?;
        let message = message.trim_end();

        self.connection
            .write_all(format!("{response}\r\n").as_bytes())
            .await?;

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
        Ok(BulbConnection { connection })
    }
}
