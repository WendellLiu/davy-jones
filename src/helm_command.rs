use std::process::Command;

pub fn helm_init() {
  let command = Command::new("helm")
    .arg("init")
    .output()
    .expect("helm command failed to start");

  println!("{}", String::from_utf8_lossy(&command.stdout));
}

pub fn helm_version() {
  let command = Command::new("helm")
    .arg("version")
    .output()
    .expect("helm command failed to start");

  println!("{}", String::from_utf8_lossy(&command.stdout));
}