use serde_json::Value;
use std::collections;
use url;

use super::super::errors;
use super::super::scope;

#[derive(Debug)]
pub enum AdditionalKind {
    Boolean(bool),
    Schema(url::Url),
}

#[allow(missing_copy_implementations)]
pub struct Properties {
    pub properties: collections::HashMap<String, url::Url>,
    pub additional: AdditionalKind,
    pub patterns: Vec<(regress::Regex, url::Url)>,
}

impl super::Validator for Properties {
    fn validate(&self, val: &Value, path: &str, scope: &scope::Scope) -> super::ValidationState {
        let object = nonstrict_process!(val.as_object(), path);
        let mut state = super::ValidationState::new();

        'main: for (key, value) in object.iter() {
            let is_property_passed = if self.properties.contains_key(key) {
                let url = &self.properties[key];
                let schema = scope.resolve(url);
                if schema.is_some() {
                    let value_path = [path, key.as_ref()].join("/");
                    state.append(schema.unwrap().validate_in(value, value_path.as_ref()))
                } else {
                    state.missing.push(url.clone())
                }

                true
            } else {
                false
            };

            let mut is_pattern_passed = false;
            for &(ref regex, ref url) in self.patterns.iter() {
                if regex.find(key.as_ref()).is_some() {
                    let schema = scope.resolve(url);
                    if schema.is_some() {
                        let value_path = [path, key.as_ref()].join("/");
                        state.append(schema.unwrap().validate_in(value, value_path.as_ref()));
                        is_pattern_passed = true;
                    } else {
                        state.missing.push(url.clone())
                    }
                }
            }

            if is_property_passed || is_pattern_passed {
                continue 'main;
            }

            match self.additional {
                AdditionalKind::Boolean(allowed) if !allowed => {
                    state.errors.push(Box::new(errors::Properties {
                        path: path.to_string(),
                        detail: "Additional properties are not allowed".to_string(),
                    }))
                }
                AdditionalKind::Schema(ref url) => {
                    let schema = scope.resolve(url);

                    if schema.is_some() {
                        let value_path = [path, key.as_ref()].join("/");
                        state.append(schema.unwrap().validate_in(value, value_path.as_ref()))
                    } else {
                        state.missing.push(url.clone())
                    }
                }
                // Additional are allowed here
                _ => (),
            }
        }

        state
    }
}
