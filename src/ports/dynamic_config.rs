use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json;

// transparent guarantees that the type is the same as the underlying representation
#[repr(transparent)]
#[derive(Deserialize)]
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

impl From<serde_json::Value> for DynamicConfig {
    fn from(v: serde_json::Value) -> Self {
        Self(v)
    }
}