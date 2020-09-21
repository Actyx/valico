use serde_json::Value;

use super::super::errors;

impl super::Validator for regress::Regex {
    fn validate(&self, val: &Value, path: &str) -> super::ValidatorResult {
        let string = strict_process!(val.as_str(), path, "The value must be a string");

        if self.find(string).is_some() {
            Ok(())
        } else {
            Err(vec![Box::new(errors::WrongValue {
                path: path.to_string(),
                detail: Some("Value is not matched by required pattern".to_string()),
            })])
        }
    }
}
