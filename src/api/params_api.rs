use actix_web::{
    delete, get,
    http::header::{self, Header, HeaderMap},
    post, put,
    web::{self, head, Data, Json, Path},
    HttpResponse,
};
use serde::Deserialize;

//typing params
#[derive(Debug, Deserialize)]
pub struct Params {
    name: String,
    age: usize,
    location: String,
}

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

#[get("/params")]
async fn get_params(params: web::Query<Params>) -> HttpResponse {
    HttpResponse::Ok().body(format!(
        "Hello {}! You are {} years old and live in {} \\n",
        params.name, params.age, params.location
    ))
}