#![feature(proc_macro_hygiene, decl_macro)]

extern crate chrono;
extern crate jsonwebtoken as jwt;

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
extern crate rocket_contrib;
extern crate handlebars;


#[cfg(not(feature = "production"))]
extern crate dotenv;

mod claims;
mod routes;
mod config;
mod response;
mod kubeconfig;

use routes::root;
use routes::webhook;
use kubeconfig::{write_kubeconfig};

fn init() {
  write_kubeconfig();
}

fn main() {
  init();

  rocket::ignite()
    .mount("/", routes![root::index, webhook::index, webhook::create_token])
    .launch();
}
