use std::collections::VecDeque;

use clap::Parser;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpSocket, TcpStream},
};

use crate::{messages::Type, wordle::Wordleizer};

mod messages;
mod wordle;

#[derive(Parser)]
struct Args {
    #[arg(short)]
    port: Option<u32>,

    #[arg(short)]
    should_use_tls: bool,

    hostname: String,

    northeastern_username: String,
}

impl Args {
    const DEFAULT_PORT: u32 = 27993;
    const DEFAULT_TLS_PORT: u32 = 27994;
    fn port(&self) -> u32 {
        self.port.unwrap_or_else(|| {
            if self.should_use_tls {
                Self::DEFAULT_TLS_PORT
            } else {
                Self::DEFAULT_PORT
            }
        })
    }
}

async fn non_tls(
    hostname: &str,
    port: u32,
    northeastern_username: &str,
) -> tokio::io::Result<String> {
    let mut buffer = [0u8; 1024];
    let mut wordleizer = Wordleizer::default();
    let mut messages_to_send = VecDeque::from([Type::Hello {
        northeastern_username: northeastern_username.to_owned(),
    }]);

    let addr = format!("{hostname}:{port}");
    println!("Connecting to: {}", addr);
    let mut connection = TcpStream::connect(addr).await?;

    let mut json_bytes = Vec::new();
    let flag = loop {
        // send queued messages
        while let Some(msg) = messages_to_send.pop_front() {
            let mut out = serde_json::to_string(&msg)?;
            out.push('\n');
            println!("Sending: {}", out);
            connection.write_all(out.as_bytes()).await?;
        }

        // read the socket until it hits a newline
        let newline_index = loop {
            if let Some(index) = json_bytes.iter().position(|byte| *byte == '\n' as u8) {
                break index;
            }


            let mut buffer = [0u8; 1024];
            let bytes_read = connection.read(&mut buffer).await?;
            if bytes_read == 0 {
                return Err(tokio::io::Error::new(
                    std::io::ErrorKind::Other,
                    "CUSTOM: No bytes read",
                ));
            }
            json_bytes.extend_from_slice(&buffer[..bytes_read]);
        };

        let server_msg: Type = serde_json::from_slice(&json_bytes[..newline_index])?;
        println!("Received: {:?}", server_msg);
        match server_msg {
            Type::Start { id } => {
                messages_to_send.push_back(Type::Guess {
                    id,
                    word: wordleizer.make_guess(),
                });
            },
            Type::Bye { flag, .. } => {
                break flag;
            }
            Type::Retry { id, guesses } => {
                todo!();
            }
            _ => {}
        }

        json_bytes.clear();
    };

    Ok(flag)
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.should_use_tls {
        //
    } else {
        let result = non_tls(&args.hostname, args.port(), &args.northeastern_username).await;
        match result {
            Ok(flag) => eprintln!("Got flag (may not be correct one though): {}", flag),
            Err(e) => eprintln!("Error from non-tls: {:?}", e),
        }
        // if result.is_err() {
        //     eprintln!("{:?}", result);
        // }
    }
}
