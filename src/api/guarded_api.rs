use actix_web::{
    delete, get,
    guard::{Guard, Header},
    post, put,
    web::{Data, Json, Path},
    HttpResponse,
};

const _SECRET: &str = "HIDDEN";
const _HEADER: &str = "X-SECRET";

// get route with name parameter

pub async fn guarded_name(path: Path<String>) -> HttpResponse {
    let name = path.into_inner();
    HttpResponse::Ok().body(format!("Hello {}!", name))
}
