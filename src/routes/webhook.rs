use rocket::http::RawStr;
use  rocket::response::status::{BadRequest};
use rocket_contrib::json::{Json};
use jwt::{decode, TokenData, Validation, encode, Header};
use crate::claims::{Claims, create_claims};
use crate::config::{get_config, Config};
use crate::response::{CustomResponse};
use crate::helm_command::{helm_delete};
use crate::utils::{concat_release, is_protected_branch};
use crate::request_guard::{WebhookSecret};

#[derive(Serialize, Deserialize)]
struct Repository {
  full_name: String
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

#[derive(Serialize, Deserialize)]
pub struct TriggerWebhookData {
  release_name: String,
  signature: String
}

#[post("/webhook/<token>", format = "json", data = "<_payload>")]
pub fn trigger_webhook(token: &RawStr, _payload: Json<DeletePayload>, webhook_secret: WebhookSecret) -> 
  Result<Json<CustomResponse<TriggerWebhookData>>, BadRequest<String>> {
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
    repository,
    ..
  } = _payload.into_inner();

  if ref_type != "branch" {
    return Err(BadRequest(Some(String::from("the ref type is not branch"))));
  }

  let token_data = decode::<Claims>(token.as_str(), secret.as_ref(), &validation);

  let claims = match token_data {
    Ok(TokenData { claims: _claims, .. }) => _claims,
    Err(e) => panic!(e),
  };

  let Claims {
    pg,
    r_pre,
    r_suf,
    repo,
    p_bran,
    hd_secr,
    ..
  } = claims;

  if repo != repository.full_name {
    return Err(BadRequest(Some(String::from("repository name does not match the payload"))));
  }

  if is_protected_branch(&branch_name, p_bran) {
    return Err(BadRequest(Some(String::from("the branch is in the protected list"))));
  }

  let WebhookSecret(signature) = webhook_secret;

  if signature != hd_secr {
    return Err(BadRequest(Some(String::from("the secret does not match"))));
  }

  let release_name = concat_release(branch_name, r_pre, r_suf);
  helm_delete(kube_tiller_ns, kube_context, pg, release_name.clone());

  let data = TriggerWebhookData {
    release_name,
    signature
  };

  Ok(Json(CustomResponse {
      status: String::from("ok"),
      data
  }))
}

#[derive(Serialize, Deserialize)]
pub struct CreateTokenPayload {
  pub release_name_prefix: Option<String>,
  pub release_name_suffix: Option<String>,
  pub purge: bool,
  pub protected_branch_name: Option<Vec<String>>,
  pub repository_name: String,
  pub webhook_secret: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateTokenData {
  token: String
}

#[post("/webhook", format = "json", data = "<_payload>")]
pub fn create_token(_payload: Json<CreateTokenPayload>) -> 
  Result<Json<CustomResponse<CreateTokenData>>, BadRequest<String>> {
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

    Ok(Json(CustomResponse {
      status: String::from("ok"),
      data
    }))
  }