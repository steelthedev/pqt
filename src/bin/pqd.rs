use std::path::Path;

use anyhow::Result;
use pqt::{Request, Response, SOCKET_PATH, read_message, write_message};
use tokio::net::UnixListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create the UDS handshake with UDS
    let path = Path::new(SOCKET_PATH);
    if path.exists() {
        std::fs::remove_file(path)?;
    }

    let listener = UnixListener::bind(path)?;

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                println!("New client connected!");
                println!("####################");
                println!("Address {:#?}", addr);
                handle_connection(stream).await?;
            }
            Err(err) => {
                eprintln!("connection error: {err}")
            }
        }
    }
}

async fn handle_connection(mut stream: tokio::net::UnixStream) -> Result<()> {
    let req: Request = read_message(&mut stream).await?;
    let res = match req {
        Request::Ping => Response::Pong,
    };
    write_message(&mut stream, &res).await?;
    println!("client said {:#?}", req);
    Ok(())
}
