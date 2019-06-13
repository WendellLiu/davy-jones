use std::io::Read;
use rocket::{Request, Data, Outcome};
use rocket::data::{self, FromDataSimple};
use rocket::http::Status;
use jwt::{decode, TokenData, Validation};
use crate::config::{get_config, Config};
use crate::claims::{Claims};



use crate::utils::{verify_signature};

const HUB_SIGNATURE: &str = "X-Hub-Signature";

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

    let validation = Validation {
      validate_exp: false,
      ..Default::default()
    };

    let config = get_config();
    let Config {
      secret,
      ..
    } = config;

    let token: String = match request.get_param(1) {
      Some(r) => r.unwrap(),
      None => return Outcome::Failure((Status::BadRequest, String::from("can not fetch params")))
    };

    let token_data = decode::<Claims>(token.as_str(), secret.as_ref(), &validation);

    let claims = match token_data {
      Ok(TokenData { claims: _claims, .. }) => _claims,
      Err(e) => panic!(e.to_string()),
    };

    let Claims {
      hd_secr,
      ..
    } = claims;

    let mut stream = data.open();
    let mut data_string = String::new();

    match stream.read_to_string(&mut data_string) {
      Err(e) => panic!(e.to_string()),
      _ => ()
    };


    if !verify_signature(&hd_secr, &String::from(&data_string), &String::from(signature)) {
      return Outcome::Failure((Status::BadRequest, String::from("the secret does not match")));
    }

    Outcome::Success(serde_json::from_str(&data_string).unwrap())
  }
}