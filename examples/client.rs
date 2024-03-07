use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "xingcheng.sdhis999.cn:37645";
    let mut stream = TcpStream::connect(&addr).await?;

    println!("Connected to server");

    let message = "Hello from client";
    stream.write_all(message.as_bytes()).await?;

    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).await?;
    let received_data = &buffer[..bytes_read];

    println!("Received: {}", String::from_utf8_lossy(received_data));

    Ok(())
}