use actix_web::{guard, web};
use actix_web::{web::Data, App, HttpServer};

mod api;
mod error;
mod model;
mod prelude;
mod repository;
mod utils;

use api::guarded_api::guarded_name;
use api::hello_api::{guarded_html, hello_name, hello_name_age_location};
use api::todo_api::{create_todo, delete_todo, get_todo, get_todos, update_todo};
use api::user_api::{
    create_user, delete_user, get_user, get_users, get_users_by_role, update_user,
};
use repository::surrealdb_repo::SurrealDBRepo;

// region -- constants
// TODO: into .env file
const _SECRET: &str = "HIDDEN";
const _HEADER: &str = "X-SECRET";
// -- end region

// region -- main

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let surreal = SurrealDBRepo::init_with_name_and_ns_and_db("local.db", "test_ns", "todo_db")
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
            .service(get_users_by_role)
            .service(update_user)
            .service(delete_user)
            .service(hello_name)
            .service(hello_name_age_location)
            .service(guarded_html)
        // .service(
        //     web::scope("/guarded")
        //         .route("/{param}", web::get().to(guarded_name))
        //         .guard(guard::Get())
        //         .guard(guard::Header(_HEADER, _SECRET)),
        // )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// -- end region
