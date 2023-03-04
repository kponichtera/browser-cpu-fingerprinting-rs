use std::error::Error;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use env_logger::Env;
use log::info;
use crate::config::read_config;
use crate::context::build_context;
use crate::handlers::register_handlers;

mod handlers;
mod config;
mod context;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let config = read_config(None);
    let context = build_context(&config).await?;

    let bind_addr = ("0.0.0.0", config.port);
    info!("Starting server on {}:{}", bind_addr.0, bind_addr.1);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(context.clone()))
            .configure(register_handlers)
    })
        .bind(bind_addr)?
        .run()
        .await?;

    return Ok(())
}
