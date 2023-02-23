mod links;
mod route_handlers;
mod state;

use std::env;
use log::info;
use actix_web::middleware::Logger;
use actix_web::{web, HttpServer, App};
use crate::state::State;
use crate::route_handlers::{index, links, add_link, rm_link};

fn init_env() {
    env::set_var("RUST_LOG", "linksnap=info");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    info!("Starting http server: 127.0.0.1:8080");
}

#[actix::main]
async fn main() -> std::io::Result<()> {
    init_env();
    let state = State::init();

    let web_app = move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .route("/links", web::get().to(links))
            .route("/add", web::post().to(add_link))
            .route("/rm", web::delete().to(rm_link))
    };
    HttpServer::new(web_app).bind("127.0.0.1:8080")?.run().await
}
