use tokio::net::UnixStream;

use anyhow::Result;
use pqt::{Request, Response, SOCKET_PATH, read_message, write_message};

#[tokio::main]
async fn main() -> Result<()> {
    let mut stream = UnixStream::connect(SOCKET_PATH).await?;
    write_message(&mut stream, &Request::Ping).await?;
    let resp: Response = read_message(&mut stream).await?;
    println!("daemon said: {resp:?}");
    Ok(())
}
