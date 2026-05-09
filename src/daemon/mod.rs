use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use tokio::net::UnixListener;

use crate::{SOCKET_PATH, daemon::scheduler::Queue, handle_connection};

pub mod scheduler;

pub async fn run() -> anyhow::Result<()> {
    let path = Path::new(SOCKET_PATH);
    if path.exists() {
        std::fs::remove_file(path)?;
    }

    let listener = UnixListener::bind(path)?;
    println!("daemon listening on {listener:?}");

    let queue = Arc::new(Mutex::new(Queue::default()));

    loop {
        match listener.accept().await {
            Ok((stream, _addr)) => {
                let queue = Arc::clone(&queue);
                handle_connection(stream, queue).await?;
            }
            Err(err) => {
                eprintln!("connection error: {err}")
            }
        }
    }
}
