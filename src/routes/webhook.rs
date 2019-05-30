use rocket::http::RawStr;
use rocket_contrib::json::{Json};
use jwt::{decode, TokenData, Validation, encode, Header};
use crate::claims::Claims;
use crate::config::{get_config, Config};
use crate::response::{CustomResponse};


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

#[derive(Serialize, Deserialize)]
pub struct CreateTokenData {
  token: String
}

#[post("/webhook", format = "json", data = "<_claims>")]
pub fn create_token(_claims: Json<Claims>) -> Option<Json<CustomResponse<CreateTokenData>>> {
  let claims = _claims.into_inner();
  
  let config = get_config();
  let Config {
    secret
  } = config;

  let token = encode(&Header::default(), &claims, secret.as_ref()).unwrap();
  let data = CreateTokenData {
    token: token
  };

  Some(Json(CustomResponse {
    status: String::from("ok"),
    data: data
  }))
}