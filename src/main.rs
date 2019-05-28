#![feature(proc_macro_hygiene, decl_macro)]

extern crate chrono;
extern crate jsonwebtoken as jwt;
extern crate pretty_env_logger;

#[macro_use]
extern crate rocket;

#[cfg(not(feature = "production"))]
extern crate dotenv;

#[cfg(not(feature = "production"))]
use dotenv::dotenv;

mod claims;
mod routes;

use routes::root;
use routes::webhook;
use chrono::prelude::*;
use claims::Claims;
use jwt::{decode, encode, Header, TokenData, Validation};
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
        hd_secr: String::from("secret"),
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

    rocket::ignite().mount("/", routes![root::index, webhook::index]).launch();
}
