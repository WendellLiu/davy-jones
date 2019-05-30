#[cfg(not(feature = "production"))]
use dotenv::dotenv;
use std::env;

pub struct Config {
  pub secret: String
}

pub fn get_config() -> Config {
  #[cfg(not(feature = "production"))]
  dotenv().ok();

  let secret = match env::var("secret") {
    Ok(val) => val,
    Err(_) => String::from("secret"),
  };

  Config {
    secret
  }
}