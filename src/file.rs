#[derive(Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct File { pub name: String, pub contents: String }
