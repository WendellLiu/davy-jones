use rocket::http::RawStr;
use rocket_contrib::json::{Json};
use jwt::{decode, TokenData, Validation, encode, Header};
use crate::claims::{Claims, create_claims};
use crate::config::{get_config, Config};
use crate::response::{CustomResponse};
use crate::helm_command::{helm_delete};
use crate::utils::{concat_release};

#[derive(Serialize, Deserialize)]
struct Repository {
  name: String
}

#[derive(Serialize, Deserialize)]
pub struct DeletePayload {
  r#ref: String,
  ref_type: String,
  repository: Repository
}

#[derive(Serialize, Deserialize)]
struct PingPayload {
  hook_id: u8
}

#[derive(Serialize, Deserialize)]
pub enum TriggerWebhookPayload {
  DeletePayload,
  PingPayload
}

#[post("/webhook/<token>", format = "json", data = "<_payload>")]
pub fn trigger_webhook(token: &RawStr, _payload: Json<DeletePayload>) -> Result<String, ()> {
  let validation = Validation {
        validate_exp: false,
        ..Default::default()
    };

  let config = get_config();
  let Config {
    secret,
    kube_tiller_ns,
    kube_context,
    ..
  } = config;

  let DeletePayload {
    r#ref: branch_name,
    ref_type,
    ..
  } = _payload.into_inner();

  let token_data = decode::<Claims>(token.as_str(), secret.as_ref(), &validation);

  let claims = match token_data {
    Ok(TokenData { claims: _claims, .. }) => _claims,
    Err(e) => panic!(e),
  };

  let Claims {
    pg,
    r_pre,
    r_suf,
    ..
  } = claims.clone();

  helm_delete(kube_tiller_ns, kube_context, pg, concat_release(branch_name, r_pre, r_suf));

  Ok(format!("{}", claims))
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