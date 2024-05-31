use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

mod api;
mod binance;
mod db;
mod models;
mod processors;
mod notifications;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_pool = db::init_pool();
    let data = web::Data::new(AppState {
        db_pool: Mutex::new(db_pool),
    });

    // Start the alert processor in a background thread
    let alert_processor = processors::AlertProcessor::new(binance::BinanceClient::new());
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(alert_processor.run());
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .configure(api::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

struct AppState {
    db_pool: Mutex<db::DbPool>,
}
