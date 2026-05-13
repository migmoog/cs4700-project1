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
        if let Some(msg) = messages_to_send.pop_front() {
            let mut out = serde_json::to_string(&msg)?;
            out.push('\n');
            println!("Sending: {}", out);
            connection.write_all(out.as_bytes()).await?;
        }

        let read_bytes = connection.read(&mut buffer).await?;
        if read_bytes == 0 {
            // eprintln!("Connection over");
            // break ">:-9".to_string();
        }
        // let Ok(msg) = serde_json::from_slice::<Type>(&buffer) else {
        //     eprintln!("recevied (len: {}){:?}", read_bytes, str::from_utf8(&buffer));
        //     continue;
        // };
        let mut last_index = 0;
        let l = json_bytes.len();
        for i in 0..l {
            last_index = i;
            if json_bytes[last_index] == '\n' as u8 {
                if last_index < l {
                    println!("Newline at {i}. {} bytes remaining", l - last_index);
                }
                break;
            }
        }
        match serde_json::from_slice::<Type>(&json_bytes[..last_index]) {
            Ok(msg) => {
                if last_index < l {
                    json_bytes.drain(0..=l);
                }
                println!("{:?}", msg);

                match msg {
                    Type::Error { message } => eprintln!("ERROR: {}", message),
                    Type::Start { id } => {
                        messages_to_send.push_back(Type::Guess {
                            id,
                            word: wordleizer.make_guess(),
                        });
                    }
                    Type::Retry { id, guesses } => {
                        break "FAKE FLAG".to_string();
                    }
                    Type::Bye { id, flag } => break flag,
                    _ => {}
                }
            }
            Err(_) => {
                json_bytes.extend_from_slice(&buffer);
            }
        }
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
        if result.is_err() {
            eprintln!("{:?}", result);
        }
    }
}
