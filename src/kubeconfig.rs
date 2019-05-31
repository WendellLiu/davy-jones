use handlebars::Handlebars;
use std::fs::{write, read_to_string};
use std::io;
use crate::config::{get_config, Config};

pub fn render_kubeconfig(template: &String) -> String {
  let reg = Handlebars::new();

  let render_result = reg.render_template(template, &json!({"name": "foo", "word": "hahaha"}));
  match render_result {
    Ok(str) => format!("{}", str),
    Err(e) => format!("error = {:?}", e),
  }
}

pub fn write_kubeconfig() -> io::Result<()> {
  let Config { kubeconfig_path, .. } = get_config();
  let template_string = read_to_string(&kubeconfig_path)?;
  let result = render_kubeconfig(&template_string);

  write(kubeconfig_path, result)
}