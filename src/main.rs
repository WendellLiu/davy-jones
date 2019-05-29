#![feature(proc_macro_hygiene, decl_macro)]

extern crate chrono;
extern crate jsonwebtoken as jwt;

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;


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
use jwt::{encode, Header};
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

    println!("example claims = {:?}", example_claims);
    let token = encode(&Header::default(), &example_claims, secret.as_ref()).unwrap();
    println!("token = {}", token);

    rocket::ignite().mount("/", routes![root::index, webhook::index, webhook::create_token]).launch();
}
