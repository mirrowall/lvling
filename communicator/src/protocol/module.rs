//! this module define the sub module structure

extern crate serialize;

use std::collections::BTreeMap;
use serialize::json::{self, Json, ToJson};

#[derive(Default, Copy, Debug, Clone)]
pub struct ModuleInfo {
  guid    : String,
  version : String,
}

impl ToJson for ModuleInfo {
  pub fn to_json(&self) -> Json {
    let mut d = BTreeMap::new();
    d.insert("guid".to_string(), self.guid.to_json());
    d.insert("version".to_string(), self.version.to_json());
    Json::Object(d)
  }
}



