#[cfg(not(feature = "production"))]
use dotenv::dotenv;
use std::env;

pub struct Config {
  pub secret: String,
  pub kubeconfig_template_path: String,
  pub kubeconfig_destination: String,
  pub kube_api_server: String,
  pub kube_namespace: String,
  pub kube_token: String,
  pub kube_tiller_ns: String,
  pub kube_context: String
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
    Err(_) => panic!(" kube api server must be given"),
  };

  let kube_namespace = match env::var("KUBE_NAMESPACE") {
    Ok(val) => val,
    Err(_) => panic!(" kube namespace must be given"),
  };

  let kube_token = match env::var("KUBE_TOKEN") {
    Ok(val) => val,
    Err(_) => String::from("token"),
  };

  let kube_tiller_ns = match env::var("KUBE_TILLER_NS") {
    Ok(val) => val,
    Err(_) => String::from("kube_tiller_ns"),
  };

  let kube_context = match env::var("KUBE_CONTEXT") {
    Ok(val) => val,
    Err(_) => String::from("kube_context"),
  };

  let current_path = match env::current_dir() {
    Ok(path) => format!("{}", path.display()),
    _ => String::from("/tmp")
  };

  let kubeconfig_destination = match cfg!(feature = "production") {
    true => String::from("/root/.kube/config"),
    false => String::from(format!("{}/.kube/config", current_path)),
  };


  Config {
    secret,
    kubeconfig_template_path: String::from("kubeconfig_template"),
    kubeconfig_destination,
    kube_api_server,
    kube_namespace,
    kube_token,
    kube_tiller_ns,
    kube_context
  }
}