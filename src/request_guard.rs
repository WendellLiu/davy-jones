use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{Request, FromRequest};
use rocket::request;

const X_GITHUB_EVENT: &str = "X-GitHub-Event";

#[derive(Serialize, Deserialize)]
pub struct GithubEvent(pub String);

impl<'a, 'r> FromRequest<'a, 'r> for GithubEvent {
  type Error = String;
  fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, String> {
    match request.headers().get_one(X_GITHUB_EVENT) {
      Some("ping") => Outcome::Forward(()),
      Some("delete") => Outcome::Success(GithubEvent(String::from("delete"))),
      _ => Outcome::Failure((Status::BadRequest, String::from("event is not allowed")))
    }
  }
}