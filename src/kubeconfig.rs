use handlebars::Handlebars;
use std::fs::{write, read_to_string, create_dir};
use std::path::Path;
use std::io;
use crate::config::{get_config, Config};

#[derive(Serialize, Deserialize)]
pub struct KubeConfigVariables {
  api_server: String,
  namespace: String,
  token: String
}

pub fn render_kubeconfig(template: &String, variables: KubeConfigVariables) -> String {
  let reg = Handlebars::new();

  let render_result = reg.render_template(template, &json!(variables));
  match render_result {
    Ok(str) => format!("{}", str),
    Err(e) => format!("error = {:?}", e),
  }
}

pub fn write_kubeconfig() -> io::Result<()> {
  let Config { 
    kubeconfig_template_path,
    kubeconfig_destination,
    kube_api_server,
    kube_namespace,
    kube_token,
    .. 
  } = get_config();

  let variables = KubeConfigVariables {
    api_server: kube_api_server,
    namespace: kube_namespace,
    token: kube_token
  };

  let template_string = read_to_string(&kubeconfig_template_path)?;
  let result = render_kubeconfig(&template_string, variables);

  let parent = match Path::new(kubeconfig_destination).parent() {
    Some(path) => path,
    None => ""
  };

  create_dir(parent)

  write(&kubeconfig_destination, result)
}