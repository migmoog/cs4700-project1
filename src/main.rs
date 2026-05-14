use std::collections::VecDeque;

use clap::Parser;
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    net::TcpStream,
};

use crate::{messages::Type, wordle::Wordleizer};

mod messages;
mod wordle;

#[derive(Parser)]
struct Args {
    #[arg(short = 'p')]
    port: Option<u32>,

    #[arg(short = 's')]
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

fn custom_error(text: &str) -> tokio::io::Result<String> {
    Err(tokio::io::Error::new(std::io::ErrorKind::Other, text))
}

async fn play<S>(northeastern_username: &str, mut connection: S) -> tokio::io::Result<String>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let mut wordleizer = Wordleizer::default();
    let mut messages_to_send = VecDeque::from([Type::Hello {
        northeastern_username: northeastern_username.to_owned(),
    }]);

    let mut json_bytes = Vec::new();
    let flag = loop {
        // send queued messages
        while let Some(msg) = messages_to_send.pop_front() {
            let mut out = serde_json::to_string(&msg)?;
            out.push('\n');
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
                return custom_error("CUSTOM: No bytes read");
            }
            json_bytes.extend_from_slice(&buffer[..bytes_read]);
        };

        // match over types of messages and perform neccessary actions
        let server_msg: Type = serde_json::from_slice(&json_bytes[..newline_index])?;
        match server_msg {
            Type::Start { id } => {
                messages_to_send.push_back(Type::Guess {
                    id,
                    word: wordleizer.make_guess(),
                });
            }
            Type::Bye { flag, .. } => {
                break flag;
            }
            Type::Retry { id, guesses } => {
                wordleizer.adjust(&guesses.last().unwrap());
                messages_to_send.push_back(Type::Guess {
                    id,
                    word: wordleizer.make_guess(),
                });
            }
            _ => {}
        }

        json_bytes.clear();
    };

    Ok(flag)
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let args = Args::parse();

    let addr = format!("{}:{}", args.hostname, args.port());
    let stream = TcpStream::connect(addr.as_str()).await?;

    let result = if args.should_use_tls {
        let args = args;
        let connector = native_tls::TlsConnector::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();
        let connector = tokio_native_tls::TlsConnector::from(connector);
        let stream = connector.connect(&args.hostname, stream).await.unwrap();
        play(&args.northeastern_username, stream).await?
    } else {
        play(&args.northeastern_username, stream).await?
    };

    println!("{}", result);

    Ok(())
}
