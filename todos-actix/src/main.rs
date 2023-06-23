use actix_web::{App, HttpServer, web};

use crate::data::AppState;

mod data;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = AppState::new();
    let app_data = web::Data::new(app_state);

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

