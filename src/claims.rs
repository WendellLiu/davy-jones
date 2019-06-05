use chrono::Utc;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
  // Release Name Prefix
  pub r_pre: Option<String>,
  // Release Name Suffix
  pub r_suf: Option<String>,
  // Purge or Not
  pub pg: bool,
  // Protected Branch Name
  pub p_bran: Option<String>,
  // Repository Name
  pub repo: String,
  // Issuer
  pub iss: String,
  // Issued At
  pub iat: chrono::DateTime<Utc>,
  // Secret for Webhook
  pub hd_secr: String,
}

// fn get_example_claims() -> Claims {
//   return Claims {
//     r_pre: Some(String::from("cweb")),
//     r_suf: None,
//     pg: true,
//     p_bran: String::from("master"),
//     repo: String::from("cweb"),
//     iss: String::from("davy-jones"),
//     iat: Utc::now(),
//     hd_secr: String::from("secret"),
//   };
// }

impl std::fmt::Display for Claims {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "issuer: {}, repo: {}", self.iss, self.repo)
  }
}

pub fn create_claims(
  release_name_prefix: Option<String>,
  release_name_suffix: Option<String>,
  purge: bool,
  protected_branch_name: Option<String>,
  repository_name: String,
  webhook_secret: String
) -> Claims {
  Claims {
    r_pre: release_name_prefix,
    r_suf: release_name_suffix,
    pg: purge,
    p_bran: protected_branch_name,
    repo: repository_name,
    iss: String::from("davy-jones"),
    iat: Utc::now(),
    hd_secr: webhook_secret,
  }
}