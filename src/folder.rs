// use super::*;
use crate as marko_fs_types;

#[derive(Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Folder { pub name: String, pub files: Vec<marko_fs_types::File>, pub subfolders: Vec<Folder> }
