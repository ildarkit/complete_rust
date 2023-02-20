use log::{info, error};
use std::env;
use std::convert::Infallible;
use hyper::Server;
use hyper::service::{make_service_fn, service_fn};

mod shortener;
mod service;
use crate::service::url_service;

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "hyperurl=info");
    env_logger::init();

    let addr = "127.0.0.1:3002".parse().unwrap();
    let make_service = make_service_fn(|_| async {
        Ok::<_, Infallible>(service_fn(url_service))
    });
    let server = Server::bind(&addr).serve(make_service);
    info!("URL shortner listening on http://{}", addr);
    if let Err(e) = server.await {
        error!("server error: {}", e);
    }
}
