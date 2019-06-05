use std::process::Command;

pub fn helm_init() {
  let command = Command::new("helm")
    .arg("init")
    .output()
    .expect("helm init failed");

  println!("{}", String::from_utf8_lossy(&command.stdout));
}

pub fn helm_version() {
  let command = Command::new("helm")
    .arg("version")
    .output()
    .expect("helm version failed");

  println!("{}", String::from_utf8_lossy(&command.stdout));
}

pub fn helm_delete(namespace: String, context: String, purge: bool, release_name: String) {
  let mut command = Command::new("helm");
  let command = command
    .arg("delete")
    .arg("--tiller-namespace")
    .arg(namespace)
    .arg("--kube-context")
    .arg(context);
  
  let command = match purge {
    true => command.arg("--purge"),
    false => command
  };

  let output = command
    .arg(release_name)
    .output()
    .expect("helm delete failed");

  println!("{}", String::from_utf8_lossy(&output.stdout));
}