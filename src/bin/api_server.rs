use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use env_logger::Env;
use log::info;
use browser_cpu_fingerprinting_rs::api_server::config::read_config;
use browser_cpu_fingerprinting_rs::api_server::handlers::register_handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let config = read_config(None);

    let bind_addr = ("0.0.0.0", config.port);
    info!("Starting server on {}:{}", bind_addr.0, bind_addr.1);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(config.clone())
            .configure(register_handlers)
    })
        .bind(bind_addr)?
        .run()
        .await
}
