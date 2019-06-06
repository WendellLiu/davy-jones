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
  Missing,
  NotAllow
}

impl<'a, 'r> FromRequest<'a, 'r> for WebhookSecret {
  type Error = WebhookError;
  fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
    let secret = match request.headers().get_one(HUB_SIGNATURE) {
      Some(s) => s,
      None => return Outcome::Failure((Status::BadRequest, WebhookError::Missing))
    };

    match request.headers().get_one(X_GITHUB_EVENT) {
      Some("ping") => Outcome::Forward(()),
      Some("delete") => Outcome::Success(WebhookSecret(secret.to_string())),
      _ => Outcome::Failure((Status::BadRequest, WebhookError::NotAllow))
    }
  }
}