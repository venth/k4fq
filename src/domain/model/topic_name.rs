use serde;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(into="String")]
pub struct TopicName {
    value: String,
}

unsafe impl Send for TopicName{}
unsafe impl Sync for TopicName{}

impl TopicName {
    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }
}

impl From<TopicName> for String {
    fn from(t: TopicName) -> Self {
        t.value
    }
}

impl From<&TopicName> for String {
    fn from(t: &TopicName) -> Self {
        t.value.clone()
    }
}

impl From<&str> for TopicName {
    fn from(value: &str) -> Self {
        Self { value: String::from(value) }
    }
}

impl From<&String> for TopicName {
    fn from(value: &String) -> Self {
        Self { value: String::from(value) }
    }
}

impl Clone for TopicName {
    fn clone(&self) -> Self {
        TopicName { value: self.value.clone() }
    }
}
