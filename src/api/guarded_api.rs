use actix_web::{
    delete, get,
    guard::Guard,
    http::header::{HeaderName, HeaderValue, InvalidHeaderName, InvalidHeaderValue, ToStrError},
    post, put,
    web::{Data, Json, Path},
    HttpResponse,
};

const _SECRET: &str = "HIDDEN";
const _HEADER: &str = "X-SECRET";

// get route with name parameter

pub async fn guarded_name(header: HeaderValue, path: Path<String>) -> HttpResponse {
    let name = path.into_inner();
    let header = header.to_str().unwrap();

    HttpResponse::Ok().body(format!("From guarded , -- arguments! {} {}", name, header))
}
