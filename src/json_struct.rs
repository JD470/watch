use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct to_watch {
    pub root: String,
    pub format: String,
    pub commands: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct key_events{
    pub events: Vec<key_event>
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct key_event{
    pub keys: Vec<String>,
    pub commands: Vec<String>
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct watch{
    pub watch: to_watch,
    pub keys: key_events
}