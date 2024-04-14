use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::io::BufReader;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::sync::oneshot::{self, Sender};

use yeetlight::*;

const PORT: u16 = 55443;
const IP_ADDR: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
const SOCKET_ADDR: SocketAddr = SocketAddr::new(IP_ADDR, PORT);

async fn expect_command(ready: Sender<()>) -> String {
    let socket = SOCKET_ADDR;
    let listener = tokio::net::TcpListener::bind(socket).await.unwrap();
    ready.send(()).unwrap();

    let (mut connection, _) = listener.accept().await.unwrap();

    let mut reader = BufReader::new(&mut connection);
    let mut message = String::new();
    reader.read_line(&mut message).await.unwrap();
    let message = message.trim_end();

    // An empty valid JSON.
    connection.write_all("{}".as_bytes()).await.unwrap();

    message.to_owned()
}

#[tokio::test]
async fn test_power_on() {
    let (tx, rx) = oneshot::channel();

    let mock_bulb = tokio::task::spawn(expect_command(tx));
    rx.await.unwrap();

    let b = Bulb::new(IP_ADDR);
    let _response = b.set_power(true, Effect::Smooth(500)).await.unwrap();

    let expected = r#"{"id":1,"method":"set_power","params":["on","smooth",500]}"#;
    let message = mock_bulb.await.unwrap();

    assert_eq!(message, expected);
}
