use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{Request, FromRequest};
use rocket::request;

const HUB_SIGNATURE: &str = "X-Hub-Signature";
const X_GITHUB_EVENT: &str = "X-GitHub-Event";

#[derive(Serialize, Deserialize)]
pub struct WebhookSecret(pub String);

#[derive(Debug)]
pub enum WebhookError {
  Missing
}

impl<'a, 'r> FromRequest<'a, 'r> for WebhookSecret {
  type Error = WebhookError;
  fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
    match request.headers().get_one(X_GITHUB_EVENT) {
      Some("ping") => return Outcome::Forward(()),
      _ => ()
    };

    match request.headers().get_one(HUB_SIGNATURE) {
      Some(s) => Outcome::Success(WebhookSecret(s.to_string())),
      None => Outcome::Failure((Status::BadRequest, WebhookError::Missing))
    }
  }
}