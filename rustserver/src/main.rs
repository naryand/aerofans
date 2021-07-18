mod db;
mod model;
mod routes;

use crate::db::Db;

use actix_web::{App, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

type StdErr = Box<dyn std::error::Error>;

#[actix_web::main]
async fn main() -> Result<(), StdErr> {
    // Load environment variables
    dotenv::dotenv().ok();

    // Connect to PostgreSQL database
    let db = Db::connect().await?;

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    // Start server
    HttpServer::new(move || App::new().data(db.clone()).service(routes::api()))
        .bind_openssl("127.0.0.1:8443", builder)?
        .run()
        .await?;

    Ok(())
}
