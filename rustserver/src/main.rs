mod db;
mod model;
mod routes;

use crate::db::Db;

use actix_web::{App, HttpServer};

type StdErr = Box<dyn std::error::Error>;

#[actix_web::main]
async fn main() -> Result<(), StdErr> {
    // Load environment variables
    dotenv::dotenv().ok();

    // Connect to PostgreSQL database
    let db = Db::connect().await?;

    // Start server
    HttpServer::new(move || App::new().data(db.clone()).service(routes::api()))
        .bind("127.0.0.1:8080")?
        .run()
        .await?;

    Ok(())
}
