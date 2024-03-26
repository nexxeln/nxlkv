use bytes::BytesMut;
use tokio::{io::AsyncReadExt, net::TcpListener};

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8081").await?;

    let mut buf = BytesMut::with_capacity(1024);
    loop {
        let (mut socket, _) = listener.accept().await?;
        socket.read_buf(&mut buf).await?;
        println!("received {:?}", buf);
    }
}
