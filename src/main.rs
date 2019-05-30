#![feature(proc_macro_hygiene, decl_macro)]

extern crate chrono;
extern crate jsonwebtoken as jwt;

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;


#[cfg(not(feature = "production"))]
extern crate dotenv;

mod claims;
mod routes;
mod config;

use routes::root;
use routes::webhook;

fn main() {
    rocket::ignite().mount("/", routes![root::index, webhook::index, webhook::create_token]).launch();
}
