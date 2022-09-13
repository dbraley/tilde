//! Default Compute@Edge template program.

use fastly::http::{header, Method, StatusCode};
use fastly::{mime, Dictionary, Error, Request, Response};
use woothee::parser::Parser;

/// The entry point for your application.
///
/// This function is triggered when your service receives a client request. It could be used to
/// route based on the request properties (such as method or path), send the request to a backend,
/// make completely new requests, and/or generate synthetic responses.
///
/// If `main` returns an error, a 500 error response will be delivered to the client.

#[fastly::main]
fn main(mut req: Request) -> Result<Response, Error> {
    printEnv("FASTLY_CACHE_GENERATION");
    printEnv("FASTLY_CUSTOMER_ID");
    printEnv("FASTLY_HOSTNAME");
    printEnv("FASTLY_POP");

    printEnv("FASTLY_REGION");
    printEnv("FASTLY_SERVICE_ID");
    printEnv("FASTLY_SERVICE_VERSION");
    printEnv("FASTLY_TRACE_ID");

    let LOCAL = std::env::var("FASTLY_HOSTNAME").unwrap() == "localhost";
    if LOCAL {
        println!("I'm Testing Locally")
    }

    let values = Dictionary::open("values");
    let strings = Dictionary::open("strings");

    println!("values[foo] = {:?}", values.get("foo"));
    println!("strings[en.welcome] = {:?}", strings.get("en.welcome"));

    // Filter request methods...
    match req.get_method() {
        // Allow GET and HEAD requests.
        &Method::GET | &Method::HEAD => (),

        // Deny anything else.
        _ => {
            return Ok(Response::from_status(StatusCode::METHOD_NOT_ALLOWED)
                .with_header(header::ALLOW, "GET, HEAD")
                .with_body_text_plain("This method is not allowed\n"))
        }
    };

    println!("About to parse UA");

    if let Some(ua) = req.get_header_str("User-Agent") {
        println!("User-Agent: {:?}", ua);
        let parser = Parser::new();
        if let Some(result) = parser.parse(&ua) {
            println!("Result: {:?}", result);
        } else {
            println!("Failed to parse User-Agent!")
        }
    } else {
        println!("Failed to get User-Agent!")
    }

    // Pattern match on the path...
    match req.get_path() {
        // If request is to the `/` path...
        "/welcome-to-compute" => {
            // Below are some common patterns for Compute@Edge services using Rust.
            // Head to https://developer.fastly.com/learning/compute/rust/ to discover more.

            // Create a new request.
            // let mut bereq = Request::get("http://httpbin.org/headers")
            //     .with_header("X-Custom-Header", "Welcome to Compute@Edge!")
            //     .with_ttl(60);

            // Add request headers.
            // bereq.set_header(
            //     "X-Another-Custom-Header",
            //     "Recommended reading: https://developer.fastly.com/learning/compute",
            // );

            // Forward the request to a backend.
            // let mut beresp = bereq.send("backend_name")?;

            // Remove response headers.
            // beresp.remove_header("X-Another-Custom-Header");

            // Log to a Fastly endpoint.
            // use std::io::Write;
            // let mut endpoint = fastly::log::Endpoint::from_name("my_endpoint");
            // writeln!(endpoint, "Hello from the edge!").unwrap();

            // Send a default synthetic response.
            Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::TEXT_HTML_UTF_8)
                .with_body(include_str!("welcome-to-compute@edge.html")))
        }

        "/" => {
            req.set_ttl(60);
            Ok(req.send("backend_a")?)
        }

        // Catch all other requests and return a 404.
        _ => Ok(Response::from_status(StatusCode::NOT_FOUND)
            .with_body_text_plain("The page you requested could not be found\n")),
    }
}

fn printEnv(val: &str) {
    println!("{}: {:?}", val, std::env::var(val));
}
