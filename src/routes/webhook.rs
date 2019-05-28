use rocket::http::RawStr;
use jwt::{decode, TokenData, Validation};
use crate::claims::Claims;


#[get("/webhook/<token>")]
pub fn index(token: &RawStr) -> String {
  let validation = Validation {
        validate_exp: false,
        ..Default::default()
    };

  let token_data = decode::<Claims>(token.as_str(), String::from("1qaz2wsx").as_ref(), &validation);

  match token_data {
      Ok(TokenData { claims, .. }) => format!("{}", claims),
      Err(e) => String::from("error"),
  }
}