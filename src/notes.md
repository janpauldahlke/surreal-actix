####1. Introduction

myself: typescript developer working for ehealth startup
curios about rust since i jumped the hype train, but a very beginner, wants to develop harder porgramming skills in backend (web stuff).

#####1.1 actix history
* Actix is a popular web framework written in Rust that provides a fast and reliable server-side application framework. It was first released in 2017 by Nikolay Kim and has since then been used by many Rust developers.
* Initially, Actix was built upon the Actor model which is a concurrency model that allows for efficient and fault-tolerant distributed systems. However, in 2019, the creator of Actix, Nikolay Kim, announced that he would no longer be maintaining the project due to disagreements with the Rust community over licensing issues.
* Following this announcement, a new community-driven fork called "Actix Web" was created in early 2020. Actix Web aims to provide a similar API to the original Actix framework while being more community-driven and having a more permissive license. 
*  *See the links to:* 
     * [github issue](https://github.com/actix/actix-web/issues/1289)
       ![image](img/about_actix_future.png)
     * [reddit](https://www.reddit.com/r/rust/comments/erdklb/actixweb_is_back_in_the_main_repo_with_a_note/)
* Actix Web has quickly gained popularity within the Rust community due to its performance, simplicity, and ease of use. It is actively maintained and has a growing ecosystem of third-party libraries and plugins.
* One of its key features is the ability to map URLs (routes) to functions and parameters
* This allows you to create custom endpoints for your application that can handle specific requests and return specific responses

#####1.2. FeatureList
  * Supports HTTP/1.x and HTTP/2.
  * Streaming and pipelining.
  * Powerful request routing with optional macros.
  * Full Tokio compatibility.
  * Keep-alive and slow requests handling.
  * Client/server WebSockets support.
  * Transparent content compression/decompression (br, gzip, deflate, zstd)
  * Multipart streams.

####2. Defining routes in Actix

* Routes are defined using the route method provided by Actix
* This method takes two arguments: a URL path and a handler function
* The URL path is a string that defines the endpoint for the route, while the handler function is the code that will be executed when the route is called

* Example: 
  * `App::new().route("/hello", web::get().to(hello_world))` or
     ```rust 
     App::new()
       .service(service_one)
       .service(service_two)
    ```

#####2.1: Handling parameters in routes

* Actix allows you to handle parameters in your routes using the `web::Path` and `web::Query` structs
* `web::Path` is used for handling parameters in the URL path, while `web::Query` is used for handling query string parameters
* Example: 
  * ```App::new().route("/user/{id}", web::get().to(get_user))```
  (In this example, the id parameter is handled using `web::Path`)

2.2: Handler Functions
* Handler functions are the code that is executed when a route is called
* They can take arguments, such as web::Path and web::Query, to handle parameters passed in the request
* Example:  

  ```rust 
  #[get("/person")]
  async fn person_route_querry(query: web::Query<PersonQuery>) -> HttpResponse {
    let result = format!(
        "route query params are, name{} {} {}",
        query.name, query.location, query.age
    );
    HttpResponse::Ok().body(result)

####3. Datastructures and Traits:

* `HttpRequest`: This is a data structure that represents an HTTP request in actix-web. It contains information about the HTTP method, headers, URL, and other metadata related to the request. [docs](https://docs.rs/actix-web/latest/actix_web/struct.HttpResponse.html)
* `HttpResponse`: This is a data structure that represents an HTTP response in actix-web. It contains the response status code, headers, and body. [docs](https://docs.rs/actix-web/latest/actix_web/struct.HttpRequest.html)
* `Route`: This is a trait that defines how to match an HTTP request to a particular handler function. It is used to define the URL path and HTTP method for a particular handler function. [docs](https://docs.rs/actix-web/latest/actix_web/struct.Route.html)
* `Hander`: This trait is used to define the behavior of request handlers in Actix web. It is implemented by functions or closures that take a request as input and return a response. Also in my findings `HttpResponse` and `HttpRequest` are also `handlers` if i dont get it wrong.
* `Guard`: This trait is used to protect matching routes of services in Actix web. It is bound to the route and defines for example, which HTTP verbs would be allowed, or if the Header matches a certain `Key:Value` field.
* `Service`:  This trait is used to define the behavior of services in Actix web. It is implemented by types that can handle multiple requests and responses, and can be shared across multiple threads or workers.


##### 3.1 not used in our examples but noteable:
* `Responder`: This trait is used to define the behavior of response objects in Actix web. It is implemented by types that can be converted into an HTTP response.
* `Middleware`: This trait is used to define the behavior of middleware in Actix web. It is implemented by types that can intercept requests and responses and modify them before passing them on to the next middleware or request handler.


####4. Testing
*  Actix-web provides a robust testing infrastructure that enables developers to write automated tests for their web applications. The Actix-web test module provides a suite of functions and macros for writing tests for individual handlers or entire applications

#####4.1 testing traits
* `TestRequest`: This trait provides methods to construct requests that can be used to test the HTTP handlers in isolation.
* `TestServer`: This trait provides a test server that can be used to test an entire web application end-to-end.
* `TestResponse`: This trait provides methods to inspect the HTTP response generated by a handler.
#####4.2 testing macros
* `actix_web::test`: This macro is used to define tests that can run inside the Actix-web test framework.
* `actix_web::test::init_service:` This macro is used to initialize a test server that can be used to test an entire application.
* `actix_web::test::call_service`: This macro is used to send a request to a test server and receive the response.

####5. asynchronous nature of web in actix
* Actix-web is built on top of the async runtime Tokio. This means that all of the network I/O in actix-web is non-blocking and fully async. When a request comes in, actix-web uses Tokio to spawn a new task to handle that request. This allows actix-web to handle a large number of requests concurrently, without blocking the event loop.

* Actix-web provides a number of built-in async middleware and utilities for working with async Rust, including:
* `actix_web::web::Data`: A container that can hold any type that implements Send and Sync, allowing for easy sharing of state between different parts of an application.
* `actix_web::web::Json`: A wrapper around serde_json for parsing JSON data in requests.
* `actix_web::web::Bytes`: A wrapper around bytes for working with binary data in requests and responses.
* `actix_web::guard::Guard`: A way to protect routes based on arbitrary conditions, such as the presence of certain headers in a request.

####6. Conclusion
1. Actix-web is a fast, lightweight, and flexible web framework for Rust.
2. Actix-web provides an easy-to-use and concise API that allows developers to build web applications with ease.
3. Actix-web leverages Rust's async/await syntax, making it a highly performant and scalable framework that is well-suited for modern web development.
4. Actix-web provides a variety of features and capabilities, including middleware, error handling, request routing, and more, making it a powerful tool for building robust web applications.
5. Actix-web's testing capabilities are also strong, with a range of testing tools and libraries available to help developers ensure their code is working as expected.
6. By using actix-web, developers can take advantage of Rust's strengths, including its performance, memory safety, and expressive type system, to build reliable, efficient, and secure web applications.
7. Actix-web is a great choice for developers who are interested in building web applications with Rust, and offers a powerful set of tools and features that make it well-suited for a wide range of use cases.

micdrop