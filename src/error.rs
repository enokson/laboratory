use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Error {
  Deserialize,
  Serialize,
  Assertion(String),
  ResultsNotFound
}

impl Error {
  pub fn to_string(&self) -> String {
    match &self {
      Error::Deserialize => "Could not deserialize state.".to_string(),
      Error::Serialize => "Could not serialize state".to_string(),
      Error::Assertion(string_ref) => string_ref.to_string(),
      Error::ResultsNotFound => "Results for the suite was not found".to_string()
    }
  }
}
