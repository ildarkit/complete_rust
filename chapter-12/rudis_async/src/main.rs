mod codec;
use crate::codec::RespCodec;

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Mutex;
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::Decoder;
use futures::stream::StreamExt;
use futures::{SinkExt, TryStreamExt, FutureExt};
use std::env;

mod commands;
use crate::commands::process_client_request;

lazy_static! {
    static ref RUDIS_DB: Mutex<HashMap<String, String>> = 
        Mutex::new(HashMap::new());
}

async fn handle_client(client: TcpStream) -> Result<(), std::io::Error> {
    let (mut tx, rx) = RespCodec.framed(client).split();
    let mut reply = rx.and_then(process_client_request);
    let task = tx.send_all(&mut reply).then(|res| async {
        if let Err(e) = res {
            eprintln!("failed to process connection; error = {:?}", e);
        }
        Ok::<(), std::io::Error>(())
    });
    tokio::spawn(task);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = env::args()
        .skip(1)
        .next()
        .unwrap_or("127.0.0.1:6378".to_string());
    let addr = addr.parse::<SocketAddr>()?;

    let listener = TcpListener::bind(&addr).await?;
    println!("rudis_async listening on: {}", addr);

    loop {
        let (client, _) = listener
            .accept()
            .await?;
        handle_client(client).await;
    }
}
