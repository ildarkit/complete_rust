mod codec;
use crate::codec::RespCodec;

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Mutex;
use anyhow::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::Decoder;
use futures::stream::StreamExt;
use futures::{SinkExt, TryFutureExt};
use std::env;

mod commands;
use crate::commands::process_client_request;

lazy_static! {
    static ref RUDIS_DB: Mutex<HashMap<String, String>> = 
        Mutex::new(HashMap::new());
}

async fn handle_client(client: TcpStream) -> Result<(), Error> {
    let (mut tx, rx) = RespCodec.framed(client).split();
    let (input, _) = rx.into_future().await;
    let input = input.unwrap().unwrap();
    let reply = process_client_request(input);
    tx.send(reply)
        .map_err(|e| {
            let msg = format!("Failed to process connection; error = {:?}", e);
            Error::msg(msg)
        })
        .await?; 
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

    while let Ok((client, addr)) = listener.accept().await {
        println!("Client connected: {:?}", addr);
        tokio::spawn(handle_client(client));
    }
    Ok(())
}
