use rocket::http::RawStr;
use rocket_contrib::json::{Json};
use jwt::{decode, TokenData, Validation, encode, Header};
use crate::claims::{Claims, create_claims};
use crate::config::{get_config, Config};
use crate::response::{CustomResponse};
use crate::helm_command::{helm_delete};


#[post("/webhook/<token>")]
pub fn index(token: &RawStr) -> String {
  let validation = Validation {
        validate_exp: false,
        ..Default::default()
    };

  let config = get_config();
  let Config {
    secret,
    ..
  } = config;

  let token_data = decode::<Claims>(token.as_str(), secret.as_ref(), &validation);

  let claims = match token_data {
    Ok(TokenData { claims: _claims, .. }) => _claims,
    Err(e) => panic!(e),
  };

  format!("{}", claims)
}

#[derive(Serialize, Deserialize)]
pub struct CreateTokenPayload {
  pub release_name_prefix: Option<String>,
  pub release_name_suffix: Option<String>,
  pub purge: bool,
  pub protected_branch_name: Option<String>,
  pub repository_name: String,
  pub webhook_secret: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateTokenData {
  token: String
}

#[post("/webhook", format = "json", data = "<_payload>")]
pub fn create_token(_payload: Json<CreateTokenPayload>) -> 
  Option<Json<CustomResponse<CreateTokenData>>> {
    let payload = _payload.into_inner();
    let claims = create_claims(
      payload.release_name_prefix, 
      payload.release_name_suffix,
      payload.purge,
      payload.protected_branch_name,
      payload.repository_name,
      payload.webhook_secret
    );
    
    let config = get_config();
    let Config {
      secret,
      ..
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