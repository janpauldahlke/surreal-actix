use actix_web::http::header;
use actix_web::HttpRequest;
//use actix_web::{get, test::call_service, test::TestRequest, web, App, HttpResponse, HttpServer};
use actix_web::{get, test::call_service, test::TestRequest, web, App, HttpResponse, HttpServer};
//use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer, Responder};

mod api;
mod error;
mod model;
mod prelude;
mod repository;
mod utils;

use api::guarded_api::guarded_name;
use api::hello_api::{guarded_html, hello_name, hello_name_age_location};
use api::params_api::{get_params, get_params_and_path};
use api::todo_api::{create_todo, delete_todo, get_todo, get_todos, update_todo};
use api::user_api::{
    create_user, delete_user, get_user, get_users, get_users_by_role, update_user,
};
use repository::surrealdb_repo::SurrealDBRepo;
use serde::Deserialize;
use yew::html::IntoHtmlResult;
use yew::prelude::*;

// region -- constants
// TODO: into .env file
const _SECRET: &str = "HIDDEN";
const _HEADER: &str = "X-SECRET";
// -- end region

// region -- main

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let surreal = SurrealDBRepo::init_with_name_and_ns_and_db("local.db", "test_ns", "todo_db")
//         .await
//         .expect("Error connecting to SurrealDB!");

//     let db_data = Data::new(surreal);

//     HttpServer::new(move || {
//         App::new()
//             .app_data(db_data.clone())
//             .service(create_todo)
//             .service(get_todos)
//             .service(get_todo)
//             .service(update_todo)
//             .service(delete_todo)
//             .service(create_user)
//             .service(get_users)
//             .service(get_user)
//             .service(get_users_by_role)
//             .service(update_user)
//             .service(delete_user)
//             .service(hello_name)
//             .service(hello_name_age_location)
//             .service(guarded_html)
//             .service(get_params_and_path)
//             .service(get_params)
//         // .service(
//         //     web::scope("/guarded")
//         //         .route("/{param}", web::get().to(guarded_name))
//         //         .guard(guard::Get())
//         //         .guard(guard::Header(_HEADER, _SECRET)),
//         // )
//     })
//     .bind(("127.0.0.1", 8080))?
//     .await
//     .run()
// }

// mod tests {
//     use super::*;
//     use actix_web::{
//         http::{self, header::ContentType},
//         test,
//     };

//     #[actix_web::test]
//     async fn test_hello() {
//         let test = hello_name;
//         let req = test::TestRequest::default()
//             .insert_header(ContentType::plaintext())
//             .to_http_request();

//         //let resp = test(req).await.unwrap();
//     }
// }

// -- end region

// a mvp of a web server
#[derive(Debug, serde::Deserialize)]
pub struct FooParams {
    name: String,
    location: String,
    age: u16,
}

// --Beginn Region: yew html!
fn yew_html() -> String {
    let html = html! {
        <div>
            <h1>{"Hello, world!"}</h1>
            <p>{"This is a test of the yew framework"}</p>
        </div>
    };
    let html_string = format!("{:?}", html.into_html_result());
    // seems like there is no eays way to render from yew macros to html?
    println!("html_string: {}", html_string.clone());
    html_string
}

fn return_yew_html() -> HttpResponse {
    let markup = yew_html();
    HttpResponse::Ok().body(markup)
}

fn plain_string_html() -> HttpResponse {
    let html = "<div>
        <h1>Hello, world!</h1>
        <p>This is a test of the the NO framework</p>
    </div>"
        .to_string();
    HttpResponse::Ok().body(html)
}

#[get("/yew")]
async fn yew_html_response() -> HttpResponse {
    return_yew_html()
    //plain_string_html()
}

// endregion

#[get("/foo")]
async fn foo_querry_params(query: web::Query<FooParams>) -> HttpResponse {
    let result = format!(
        "route params are, name{} {} {}",
        query.name, query.location, query.age
    );
    HttpResponse::Ok().body(result)
}
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

async fn greet(name: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().body(format!("Hello, {}!", name))
}

async fn greet_handler(req: HttpRequest, path: web::Path<String>) -> HttpResponse {
    let name = path.into_inner();
    if name.len() > 5 {
        return HttpResponse::Ok().body(format!("Hello {:?}", name));
    } else {
        return HttpResponse::BadRequest().into();
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server =
        HttpServer::new(|| App::new().service(yew_html_response)).bind("127.0.0.1:8080")?;

    server.run().await
}

#[cfg(test)]
mod tests {

    use super::*;
    use actix_web::{test, web, App};

    #[actix_web::test]
    async fn test_foo_querry_params() {
        let app = test::init_service(App::new().service(foo_querry_params)).await;
        let request = TestRequest::with_uri("/foo?name=actix&location=world&age=43").to_request();

        let response = call_service(&app, request).await;
        assert!(response.status().is_success());

        let body = test::read_body(response).await;
        println!("{:?}", body.clone()); //borrowed
        assert_eq!(body, "route params are, nameactix world 43");
    }
}
