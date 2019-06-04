#[cfg(not(feature = "production"))]
use dotenv::dotenv;
use std::env;

pub struct Config {
  pub secret: String,
  pub kubeconfig_template_path: String,
  pub kubeconfig_path: String,
  pub kube_api_server: String,
  pub kube_namespace: String,
  pub kube_token: String,
}

pub fn get_config() -> Config {
  #[cfg(not(feature = "production"))]
  dotenv().ok();

  let secret = match env::var("SECRET") {
    Ok(val) => val,
    Err(_) => String::from("secret"),
  };

  let kube_api_server = match env::var("KUBE_API_SERVER") {
    Ok(val) => val,
    Err(_) => String::from("api_server"),
  };

  let kube_namespace = match env::var("KUBE_NAMESPACE") {
    Ok(val) => val,
    Err(_) => String::from("namespace"),
  };

  let kube_token = match env::var("KUBE_TOKEN") {
    Ok(val) => val,
    Err(_) => String::from("token"),
  };


  Config {
    secret,
    kubeconfig_template_path: String::from("kubeconfig_template"),
    kubeconfig_path: String::from("kubeconfig"),
    kube_api_server,
    kube_namespace,
    kube_token,
  }
}