use handlebars::Handlebars;
use std::fs;
use std::io;
use crate::config::{get_config, Config};

pub fn render_kubeconfig(template: &String) -> String {
  let reg = Handlebars::new();

  let render_result = reg.render_template(template, &json!({"name": "foo"}));
  match render_result {
    Ok(str) => format!("{}", str),
    Err(e) => format!("error = {:?}", e),
  }
}

pub fn write_kubeconfig() -> io::Result<()> {
  let Config { kubeconfig_path, .. } = get_config();
  fs::write(kubeconfig_path, b"Lorem ipsum")
}