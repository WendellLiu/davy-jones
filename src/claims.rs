use chrono::Utc;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
  // Release Name Prefix
  pub r_pre: Option<String>,
  // Release Name Suffix
  pub r_suf: Option<String>,
  // Purge or Not
  pub pg: bool,
  // Protected Branch Name
  pub p_bran: String,
  // Repository Name
  pub repo: String,
  // Issuer
  pub iss: String,
  // Issued At
  pub iat: chrono::DateTime<Utc>,
}

impl std::fmt::Display for Claims {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "issuer: {}, repo: {}", self.iss, self.repo)
  }
}
