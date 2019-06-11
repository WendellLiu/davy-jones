use rocket::{Request, Data, Outcome, Outcome::*};
use rocket::data::{self, FromDataSimple};
use std::str::from_utf8;

const HUB_SIGNATURE: &str = "X-Hub-Signature";
const X_GITHUB_EVENT: &str = "X-GitHub-Event";

#[derive(Serialize, Deserialize, Debug)]
pub struct Repository {
  pub full_name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeletePayload {
  pub r#ref: String,
  pub ref_type: String,
  pub repository: Repository
}

impl FromDataSimple for DeletePayload {
  type Error = String;
  fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {
    let data_string = from_utf8(data.peek()).unwrap();
    Outcome::Success(serde_json::from_str(data_string).unwrap())
  }
}