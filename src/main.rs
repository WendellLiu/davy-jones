#![feature(proc_macro_hygiene, decl_macro)]

extern crate chrono;
extern crate jsonwebtoken as jwt;

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
extern crate pretty_env_logger;
extern crate rocket_contrib;
extern crate handlebars;
extern crate crypto;
extern crate itertools;


#[cfg(not(feature = "production"))]
extern crate dotenv;

mod claims;
mod routes;
mod config;
mod response;
mod kubeconfig;
mod helm_command;
mod utils;
mod data_guard;

use routes::root;
use routes::webhook;
use kubeconfig::{write_kubeconfig};
use helm_command::{helm_init, helm_version};

fn init() {
  write_kubeconfig();
  helm_version();
  helm_init();
}

fn main() {
  pretty_env_logger::init();

  init();

  rocket::ignite()
    .mount("/", routes![root::index, webhook::trigger_webhook, webhook::ping_webhook, webhook::create_token])
    .launch();
}
