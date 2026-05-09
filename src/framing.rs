use std::sync::{Arc, Mutex};

use anyhow::Result;
use serde::{Serialize, de::DeserializeOwned};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::UnixStream,
};

use crate::{JobSummary, Request, Response, scheduler::Queue};

pub async fn write_message<W, T>(writer: &mut W, msg: &T) -> Result<()>
where
    W: AsyncWriteExt + Unpin,
    T: Serialize,
{
    let bytes = serde_json::to_vec(msg)?;
    let len = u32::try_from(bytes.len())?.to_be_bytes();
    writer.write_all(&len).await?;
    writer.write_all(&bytes).await?;
    writer.flush().await?;
    Ok(())
}

pub async fn read_message<R, T>(reader: &mut R) -> Result<T>
where
    R: AsyncReadExt + Unpin,
    T: DeserializeOwned,
{
    let mut len_buf = [0u8; 4];
    reader.read_exact(&mut len_buf).await?;
    let len = u32::from_be_bytes(len_buf) as usize;

    let mut payload = vec![0u8; len];
    reader.read_exact(&mut payload).await?;
    Ok(serde_json::from_slice(&payload)?)
}

pub async fn handle_connection(mut stream: UnixStream, queue: Arc<Mutex<Queue>>) -> Result<()> {
    let req: Request = read_message(&mut stream).await?;

    let resp = match req {
        Request::Ping => Response::Pong,

        Request::Add { cmd, cwd } => {
            let id = queue.lock().unwrap().add(cmd, cwd);
            Response::Added { id }
        }
        Request::List => {
            let jobs: Vec<JobSummary> = queue.lock().unwrap().list();
            Response::Jobs { jobs }
        }
    };

    write_message(&mut stream, &resp).await?;

    Ok(())
}
