use actix_web::{
    delete, get,
    http::header::{self, Header, HeaderMap},
    post, put,
    web::{self, head, Data, Json, Path},
    Handler, HttpResponse, ResponseError,
};
use serde::Deserialize;

//typing params

// desired route is /params?name=John&age=20&location=USA
// get route with name parameter
#[get("/params/{one}")]
async fn get_params_and_path(path: Path<String>, params: web::Query<Params>) -> HttpResponse {
    let path = path.into_inner();
    HttpResponse::Ok().body(format!(
        "Hello {}! You are {} years old and live in {} \\n
        Path {}, 
        ",
        params.name, params.age, params.location, path
    ))
}

// --region overcome the error

#[derive(Debug)]
//https://stackoverflow.com/questions/67811502/rust-actix-web-the-trait-handler-is-not-implemented
pub struct MyError(String); // <-- needs debug and display

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A validation error occured on the input.")
    }
}
impl ResponseError for MyError {} // <-- key

// #[get("/params")]
// async fn get_params(params: web::Query<Params>) -> Result<HttpResponse, MyError> {
//     let response = HttpResponse::Ok().body(format!(
//         "Hello {}! You are {} years old and live in {}",
//         params.name, params.age, params.location
//     ));

//     Ok(response)
// }
// end region

#[derive(Debug, Deserialize)]
pub struct Params {
    name: String,
    age: usize,
    location: String,
}

#[get("/params")]
async fn get_params(params: web::Query<Params>) -> HttpResponse {
    HttpResponse::Ok().body(format!(
        "Hello {}! You are {} years old and live in {}",
        params.name, params.age, params.location
    ))
}

//not useful tests, that are not runnig because of the Handler Error

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use actix_web::{http::header::ContentType, test, web, App, ResponseError};

//     #[actix_web::test]
//     async fn test_get_params() {
//         let page = get_params;

//         let app = test::init_service(
//             App::new().service(
//                 web::resource("/params?name=John&age=30&location=New%20York")
//                     .route(web::get().to(page)),
//             ),
//         )
//         .await;

//         let req = test::TestRequest::default()
//             .insert_header(ContentType::plaintext())
//             .uri("/params?name=John&age=30&location=New%20York")
//             .to_request();

//         let resp = test::call_service(&app, req).await;

//         assert!(resp.status().is_success());
//         let body = test::read_body(resp).await;
//         assert_eq!(
//             body,
//             "Hello John! You are 30 years old and live in New York"
//         );
//     }
// }
