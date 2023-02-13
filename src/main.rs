use actix_web::{web::Data, App, HttpServer};

mod api;
mod error;
mod model;
mod prelude;
mod repository;
mod utils;

use repository::surrealdb_repo::SurrealDBRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let surreal = SurrealDBRepo::init()
        .await
        .expect("Error connecting to SurrealDB!");
    let db_data = Data::new(surreal);

    HttpServer::new(move || App::new().app_data(db_data.clone()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
