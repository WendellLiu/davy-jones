extern crate chrono;
extern crate futures;
extern crate hyper;
extern crate jsonwebtoken as jwt;
extern crate pretty_env_logger;

#[cfg(not(feature = "production"))]
extern crate dotenv;

#[cfg(not(feature = "production"))]
use dotenv::dotenv;

mod claims;
mod routes;

use chrono::prelude::*;
use claims::Claims;
use hyper::rt::{self, Future};
use hyper::service::service_fn;
use hyper::Server;
use jwt::{decode, encode, Header, TokenData, Validation};
use routes::router::echo;
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

    let validation = Validation {
        validate_exp: false,
        ..Default::default()
    };

    let token_data = decode::<Claims>(&token, secret.as_ref(), &validation);

    match token_data {
        Ok(TokenData { claims, .. }) => println!("decoded claims = {:?}", claims),
        Err(e) => println!("fail to parse the token, error = {:?}", e),
    }

    pretty_env_logger::init();
    let addr = ([127, 0, 0, 1], 8000).into();

    let server = Server::bind(&addr)
        .serve(|| service_fn(echo))
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);

    rt::run(server);
}
