use actix_web::{web::Data, App, HttpServer};

mod api;
mod error;
mod model;
mod prelude;
mod repository;
mod utils;

use api::todo_api::{create_todo, delete_todo, get_todo, get_todos, update_todo};
use api::user_api::{create_user, delete_user, get_user, get_users, update_user};
use repository::surrealdb_repo::SurrealDBRepo; // Add

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let surreal = SurrealDBRepo::init()
        .await
        .expect("Error connecting to SurrealDB!");

    let db_data = Data::new(surreal);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(create_todo)
            .service(get_todos)
            .service(get_todo)
            .service(update_todo)
            .service(delete_todo)
            .service(create_user)
            .service(get_users)
            .service(get_user)
            .service(update_user)
            .service(delete_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
