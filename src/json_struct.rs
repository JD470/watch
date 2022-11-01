use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ToWatch {
    pub root: String,
    pub format: String,
    pub commands: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Watch{
    pub watch: ToWatch
}