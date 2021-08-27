mod db;
mod model;
mod routes;

use crate::db::Db;

use actix_cors::Cors;
use actix_web::{App, HttpServer};

type StdErr = Box<dyn std::error::Error>;

#[actix_web::main]
async fn main() -> Result<(), StdErr> {
    // Load environment variables
    dotenv::dotenv().ok();

    // Connect to PostgreSQL database
    let db = Db::connect().await?;

    // Start server
    HttpServer::new(move || {
        
        // Add PERMISSIVE CORS controls
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .data(db.clone())
            .configure(routes::config)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await?;

    Ok(())
}
