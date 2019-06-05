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