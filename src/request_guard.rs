use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{Request, FromRequest};
use rocket::request;

const HUB_SIGNATURE: &str = "X-Hub-Signature";

#[derive(Serialize, Deserialize)]
pub struct WebhookSecret(String);

#[derive(Debug)]
pub enum ApiKeyError {
  Missing
}

impl<'a, 'r> FromRequest<'a, 'r> for WebhookSecret {
  type Error = ApiKeyError;
  fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
    match request.headers().get_one(HUB_SIGNATURE) {
      Some(s) => Outcome::Success(WebhookSecret(s.to_string())),
      None => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing))
    }

    
    // match keys.len() {
    //     0 => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
        // 1 if is_valid(keys[0]) => Outcome::Success(ApiKey(keys[0].to_string())),
    //     1 => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
    //     _ => Outcome::Failure((Status::BadRequest, ApiKeyError::BadCount)),
    // }
  }
}