use actix_web::{get, test::call_service, test::TestRequest, web, App, HttpResponse, HttpServer};

#[get("/hello")]
async fn hello(params: web::Query<Params>) -> HttpResponse {
    HttpResponse::Ok().body(format!("Hello, {}!", params.name))
}

#[derive(Debug, serde::Deserialize)]
struct Params {
    name: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| App::new().service(hello)).bind("127.0.0.1:8080")?;
    server.run().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::header::ContentType, test, web, App};

    #[actix_web::test]
    async fn test_hello() {
        let app = test::init_service(App::new().service(hello)).await;
        let req = TestRequest::with_uri("/hello?name=foo").to_request();
        let resp = call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        assert_eq!(body, "Hello, foo!");
    }
}
