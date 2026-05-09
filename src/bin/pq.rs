use std::env;

use clap::{Parser, Subcommand};
use tokio::net::UnixStream;

use anyhow::Result;
use pqt::{Request, Response, SOCKET_PATH, read_message, write_message};

#[derive(Parser)]
#[command(name = "pq")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Ping,
    Add {
        #[arg(trailing_var_arg = true, required = true)]
        cmd: Vec<String>,
    },
    List,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Translate the cli command to a wire Request

    let request = match cli.command {
        Command::Ping => Request::Ping,
        Command::Add { cmd } => Request::Add {
            cmd,
            cwd: env::current_dir()?,
        },
        Command::List => Request::List,
    };

    // one connection, one request, one Response, exit.
    let mut stream = UnixStream::connect(SOCKET_PATH).await?;
    write_message(&mut stream, &request).await?;
    let response: Response = read_message(&mut stream).await?;

    match response {
        Response::Pong => println!("Pong"),
        Response::Added { id } => println!("Queued Job {id}"),
        Response::Jobs { jobs } => {
            if jobs.is_empty() {
                println!("no jobs")
            } else {
                println!("{:<4} {:<8} COMMAND", "ID", "STATUS");
                for job in jobs {
                    println!(
                        "{:<4} {:<8} {}",
                        job.id,
                        format!("{:?}", job.status).to_lowercase(),
                        job.cmd.join(" "),
                    );
                }
            }
        }
        Response::Error { message } => {
            eprintln!("error: {message}");
            std::process::exit(1);
        }
    }

    Ok(())
}
