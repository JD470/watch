use nanoserde::{DeJson};

#[derive(Debug, DeJson, Clone)]
#[nserde(rename_all = "snake_case")]
pub struct ToWatch {
    pub root: String,
    pub format: String,
    pub commands: Vec<String>,
}

#[derive(Debug, DeJson, Clone)]
#[nserde(rename_all = "snake_case")]
pub struct KeyEvents{
    pub events: Vec<KeyEvent>
}

#[derive(Debug, DeJson, Clone)]
#[nserde(rename_all = "snake_case")]
pub struct KeyEvent{
    pub keys: Vec<String>,
    pub commands: Vec<String>
}

#[derive(Debug, DeJson, Clone)]
#[nserde(rename_all = "snake_case")]
pub struct Watch{
    pub watch: ToWatch,
    pub keys: KeyEvents
}