use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use actix_web::http::header;
use actix_web::middleware::Logger;

use crate::data::AppState;

mod data;
mod handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const RUST_LOG: &str = "RUST_LOG";
    if std::env::var_os(RUST_LOG).is_none() {
        std::env::set_var(RUST_LOG, "actix_web=info");
    }
    env_logger::init();

    let app_state = AppState::new();
    let app_data = web::Data::new(app_state);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:4200")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(app_data.clone())
            .configure(handler::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

