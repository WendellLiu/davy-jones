use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha1::Sha1;
use itertools::Itertools;

pub fn concat_release(branch_name: String, prefix: Option<String>, suffix: Option<String>) -> String {
  let pref = match prefix {
    Some(str) => str,
    None => String::from("")
  };

  let suff = match suffix {
    Some(str) => str,
    None => String::from("")
  };

  format!("{}{}{}",pref, branch_name, suff)
}

#[cfg(test)]
mod concat_release_tests {
  use super::*;

  #[test]
  fn test_happy_case() {
    assert_eq!(
      concat_release(String::from("branch"), Some(String::from("foo_")), Some(String::from("_bar"))), 
      "foo_branch_bar"
    );
  }

  #[test]
  fn test_non_prefix() {
    assert_eq!(
      concat_release(String::from("branch"), None, Some(String::from("_bar"))), 
      "branch_bar"
    );
  }

  #[test]
  fn test_non_suffix() {
    assert_eq!(
      concat_release(String::from("branch"), Some(String::from("foo_")), None), 
      "foo_branch"
    );
  }
}

pub fn is_protected_branch(branch_name: &String, protected_branches: Option<Vec<String>>) -> bool {
  match protected_branches {
    Some(p_bran) => p_bran.contains(branch_name),
    None => false
  }
}

#[cfg(test)]
mod is_protected_branch_tests {
  use super::*;

  #[test]
  fn test_happy_case() {
    assert_eq!(
      is_protected_branch(&String::from("foo"), Some(vec![String::from("foo")])), 
      true
    );
  }

  #[test]
  fn test_non_protected_branches() {
    assert_eq!(
      is_protected_branch(&String::from("foo"), None), 
      false
    );
  }

  #[test]
  fn test_empty_protected_branches() {
    assert_eq!(
      is_protected_branch(&String::from("foo"), Some(vec![])), 
      false
    );
  }
}

pub fn verify_signature(key: &String, message: &String, signature: &String) -> bool {
  let mut mac= Hmac::new(Sha1::new(), key.as_bytes());
  mac.input(message.as_bytes());
  let result = mac.result();
  let code = result.code();
  let code = code.iter().format_with("", |byte, f| f(&format_args!("{:02x}", byte))).to_string();
  &code == signature
}

#[cfg(test)]
mod verify_signature_tests {
  use super::*;
  const BODY_CONTENT: &str = "bodystring";
  const KEY: &str = "secret_key";
  const COMPUTED_HMAC: &str = "97049623b0e5d20bf6beb5313d80600e3d6abe56";

  #[test]
  fn test_happy_case() {
    assert_eq!(
      verify_signature(&String::from(KEY), &String::from(BODY_CONTENT), &String::from(COMPUTED_HMAC)), 
      true
    );
  }
}