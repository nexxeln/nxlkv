use tokio::net::TcpStream;

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let _stream = TcpStream::connect("127.0.0.1:8081").await?;
    Ok(())
}
