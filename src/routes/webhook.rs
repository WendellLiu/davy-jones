use rocket::http::RawStr;
use rocket_contrib::json::{Json};
use rocket::response::status;
use jwt::{decode, TokenData, Validation, encode, Header};
use crate::claims::Claims;


#[get("/webhook/<token>")]
pub fn index(token: &RawStr) -> String {
  let validation = Validation {
        validate_exp: false,
        ..Default::default()
    };
  let secret = String::from("1qaz2wsx");

  let token_data = decode::<Claims>(token.as_str(), secret.as_ref(), &validation);

  match token_data {
      Ok(TokenData { claims, .. }) => format!("{}", claims),
      Err(e) => format!("error = {:?}", e),
  }
}


struct Response {
  status: String,
  token: String
}

#[post("/webhook", format = "json", data = "<claims>")]
pub fn create_token(claims: Json<Claims>) -> Result<Response, status::BadRequest<String>> {
  println!("create the claims = {:?}", claims);
  let secret = String::from("1qaz2wsx");

  let token = encode(&Header::default(), &claims, secret.as_ref()).unwrap();
  Ok(Response {
    status: String::from("ok"),
    token: token
  })
}