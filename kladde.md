### 1. Mapping URLs (routes) to functions and parameters
Actix provides a simple way to map URLs to functions and parameters using the Route struct. Here's an example:

```rust
use actix_web::{web, App, HttpResponse, HttpServer};

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

async fn greet(name: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().body(format!("Hello, {}!", name))
}

pub struct FooParams {
    name: String,
    location: String,
    age: u16,
}
async fn foo_querry_params(query: web::Query<FooParams>) -> HttpResponse {
    let result = format!(
        "route params are, name{} {} {}",
        query.name, query.location, query.age
    );
    HttpResponse::Ok().body(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || { // why move keyword ?
        App::new()
            .route("/", web::get().to(index))
            .route("/hello/{name}", web::get().to(greet))
            .route("/foobar")
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

```
In this example, the index function is mapped to the root URL ("/"), and the greet function is mapped to "/hello/{name}". The {name} parameter is extracted using the web::Path extractor.

### 2. Important data structures and traits

Actix comes with several important data structures and traits that make it easy to build web applications. Some of the most important ones include:

HttpRequest and HttpResponse: These are the main request and response types used in Actix.

App: This is the main application type in Actix. It's used to configure the application and route incoming requests.

Route: This is used to map URLs to functions and parameters.

web::Data: This is a container for shared state that can be accessed by multiple handlers.

web::Json: This is an extractor that automatically deserializes JSON from the request body.

AsyncResponder: This trait is used to define async handlers.


### 3. Infrastructure for automatic testing
Actix provides a testing infrastructure that makes it easy to test your web application. Here's an example:

```rust
use actix_web::{test, App, HttpResponse};

#[actix_rt::test]
async fn test_index() {
    let app = App::new().route("/", web::get().to(|| HttpResponse::Ok()));

    let req = test::TestRequest::with_uri("/").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
}
```
In this example, we're testing the index function by creating an App instance with a single route that returns an HttpResponse::Ok. We then create a TestRequest and use the call_service function to execute it against the app. Finally, we assert that the response status is successful.

### 4. Working with async Rust

```rust
use actix_web::{get, web, App, HttpResponse, HttpServer};

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

In this example, the index function is defined as async and returns an HttpResponse. The App instance is configured with a single route that maps




