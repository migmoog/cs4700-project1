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
    mut buffer: Vec<u8>,
) -> tokio::io::Result<String> {
    let mut wordleizer = Wordleizer::default();
    let mut messages_to_send = VecDeque::from([Type::Hello {
        northeastern_username: northeastern_username.to_owned(),
    }]);

    let addr = format!("{hostname}:{port}");
    println!("Connecting to: {}", addr);
    let mut connection = TcpStream::connect(addr).await?;
    let flag = loop {
        if let Some(msg) = messages_to_send.pop_front() {
            let mut out = serde_json::to_string(&msg)?;
            out.push('\n');
            println!("{}", out);
            connection.write_all(out.as_bytes()).await?;
        }

        let read_bytes = connection.read(&mut buffer).await?;
        if read_bytes == 0 {
            eprintln!("Connection over");
            break ">:-9".to_string();
        }
        let Ok(msg) = serde_json::from_slice::<Type>(&buffer) else {
            eprintln!("(len: {}){:?}", read_bytes, str::from_utf8(&buffer));
            continue;
        };

        println!("{:?}", serde_json::to_string(&msg));

        match msg {
            Type::Error { message } => eprintln!("{}", message),
            Type::Start { id } => {
                messages_to_send.push_back(Type::Guess {
                    id,
                    word: wordleizer.make_guess(),
                });
            }
            Type::Retry { id, guesses } => {
                eprintln!("{:?}", guesses);
                break "FAKE FLAG".to_string();
            }
            Type::Bye { id, flag } => break flag,
            _ => {}
        }
    };

    Ok(flag)
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let buffer: Vec<u8> = vec![];
    if args.should_use_tls {
        //
    } else {
        let result = non_tls(
            &args.hostname,
            args.port(),
            &args.northeastern_username,
            buffer,
        )
        .await;
        if result.is_err() {
            eprintln!("{:?}", result);
        }
    }
}
