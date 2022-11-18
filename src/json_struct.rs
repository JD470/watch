use nanoserde::{DeJson};

#[derive(Debug, DeJson, Clone)]
#[nserde(rename_all = "snake_case")]
pub struct KeyEvent{
    pub keys: Vec<String>,
    pub commands: Vec<String>
}

#[derive(Debug, DeJson, Clone)]
#[nserde(rename_all = "snake_case")]
pub struct ToWatch {
    pub root: Vec<String>,
    pub format: Vec<String>,
    pub commands: Vec<String>
}

#[derive(Debug, DeJson, Clone)]
#[nserde(rename_all = "snake_case")]
pub struct Watch{
    pub to_watch: ToWatch,
    pub key_events: Vec<KeyEvent>
}