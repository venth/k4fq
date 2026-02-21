use std::fmt::Display;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json;

// transparent guarantees that the type is the same as the underlying representation
#[repr(transparent)]
#[derive(Deserialize, Serialize)]
#[derive(Debug, Clone)]
pub struct DynamicConfig(serde_json::Value);

impl DynamicConfig {
    pub fn new() -> Self {
        Self(serde_json::Value::Null)
    }

    pub fn merge(&self, other: DynamicConfig) -> Self {
        fn deep_merge_recursive(target: &mut serde_json::Value, source: serde_json::Value) {
            match (target, source) {
                (serde_json::Value::Object(t), serde_json::Value::Object(s)) => {
                    for (k, v) in s {
                        deep_merge_recursive(t.entry(k).or_insert(serde_json::Value::Null), v);
                    }
                }
                (t, s) => *t = s,
            }
        }
        let mut result = self.0.clone();
        deep_merge_recursive(&mut result, other.0);

        Self(result)
    }

    pub fn try_into_struct<T: DeserializeOwned>(self) -> Result<T, serde_json::Error> {
        serde_json::from_value(self.0)
    }
}

impl Display for DynamicConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_yaml::to_string(&self.0).unwrap())
    }
}

impl From<serde_json::Value> for DynamicConfig {
    fn from(v: serde_json::Value) -> Self {
        Self(v)
    }
}