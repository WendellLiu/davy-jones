use rocket::{Request, Data, Outcome};
use rocket::data::{self, FromDataSimple};
use rocket::http::Status;
use std::str::from_utf8;

use crate::utils::{verify_signature};

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

  fn from_data(request: &Request, data: Data) -> data::Outcome<Self, String> {
    let signature = match request.headers().get_one(HUB_SIGNATURE) {
      Some(s) => s,
      None => return Outcome::Failure((Status::BadRequest, String::from("no signature")))
    };

    // let claims = request.get_param(0);

    // if !verify_signature(&hd_secr, &data, &signature) {
    //   return Err(BadRequest(Some(String::from("the secret does not match"))));
    // }

    let data_string = from_utf8(data.peek()).unwrap();
    Outcome::Success(serde_json::from_str(data_string).unwrap())
  }
}