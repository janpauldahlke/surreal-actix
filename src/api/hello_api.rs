use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};

// get route with name parameter
#[get("/hello/{name}")]
pub async fn hello_name(path: Path<String>) -> HttpResponse {
    let name = path.into_inner();
    HttpResponse::Ok().body(format!("Hello {}!", name))
}

//get route with name, age location paramter
#[get("/hello/{name}/{age}/{location}")]
pub async fn hello_name_age_location(path: Path<(String, u32, String)>) -> HttpResponse {
    let (name, age, location) = path.into_inner();
    HttpResponse::Ok().body(format!(
        "Hello {}! You are {} years old and you live in {}",
        name, age, location
    ))
}