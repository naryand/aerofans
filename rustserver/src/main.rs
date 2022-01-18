mod controller;
mod model;

use std::time::Duration;

use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use model::init;
use sea_orm::{ConnectOptions, Database};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::dotenv().ok();

    // Connect to PostgreSQL database
    let db_url = std::env::var("DATABASE_URL")?;
    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .sqlx_logging(true);

    let pool = Data::new(Database::connect(opt).await?);
    init(pool.as_ref()).await;

    // Start server
    HttpServer::new(move || {
        // Add PERMISSIVE CORS controls
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(pool.clone())
            .configure(controller::config)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await?;

    Ok(())
}
