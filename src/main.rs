extern crate chrono;
extern crate hyper;
extern crate jsonwebtoken as jwt;
extern crate pretty_env_logger;

#[cfg(not(feature = "production"))]
extern crate dotenv;

#[cfg(not(feature = "production"))]
use dotenv::dotenv;

mod claims;

use chrono::prelude::*;
use claims::Claims;
use hyper::rt::{self, Future};
use hyper::service::service_fn_ok;
use hyper::{Body, Request, Response, Server};
use jwt::{decode, encode, Header};
use std::env;

fn main() {
    #[cfg(not(feature = "production"))]
    dotenv().ok();

    let secret = match env::var("secret") {
        Ok(val) => val,
        Err(_) => String::from("secret"),
    };

    println!("secret = {}", secret);

    let example_claims: Claims = Claims {
        r_pre: Some(String::from("cweb")),
        r_suf: None,
        pg: true,
        p_bran: String::from("master"),
        repo: String::from("cweb"),
        iss: String::from("davy-jones"),
        iat: Utc::now(),
    };

    let token = encode(&Header::default(), &example_claims, secret.as_ref()).unwrap();
    println!("token = {}", token);

    let serialized = serde_json::to_string(&example_claims).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: Claims = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
    pretty_env_logger::init();
    let addr = ([127, 0, 0, 1], 8000).into();

    let server = Server::bind(&addr)
        .serve(|| {
            // This is the `Service` that will handle the connection.
            // `service_fn_ok` is a helper to convert a function that
            // returns a Response into a `Service`.
            service_fn_ok(move |_: Request<Body>| Response::new(Body::from("Hello World!!!")))
        })
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);

    rt::run(server);
}
