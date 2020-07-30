//! this module define the client struct

extern crate serialize;

use std::collections::BTreeMap;
use serialize::json::{self, Json, ToJson};

#[derive(Default,Debug)]
pub struct ClientInfo {
  /// define the computer name
  name    : String,
  /// the computer arch
  arch    : String,
  /// the ip address
  ip      : String,
  /// if it is a virtual machine
  isvm    : i32,
  /// mac address
  mac     : String,
  /// the os name, like Linux or Windows
  os      : String,
  /// the system root dictionary
  root    : String,
  /// the memory size
  memory  : String,
  /// the disk size
  disk    : String,
  /// the usernmae
  user    : String,
  /// the version
  version : String
}

impl Default for ClientInfo {
  fn default() -> ClientInfo {
    ClientInfo::new()
  }
}

impl ClientInfo {
  fn new() -> ClientInfo {
    ClientInfo{}
  }
}

impl ToJson for ClientInfo {
  pub fn to_json(&self) -> Json {
    let mut d = BTreeMap::new();
    d.insert("computername".to_string(), self.name.to_json());
    d.insert("cpuarchitecture".to_string(), self.arch.to_json());
    d.insert("ip".to_string(), self.ip.to_json());
    d.insert("isvm".to_string(), self.isvm.to_json());
    d.insert("mac".to_string(), self.mac.to_json());
    d.insert("os".to_string(), self.os.to_json());
    d.insert("systemdirectory".to_string(), self.root.to_json());
    d.insert("totalmemory".to_string(), self.memory.to_json());
    d.insert("username".to_string(), self.user.to_json());
    d.insert("version".to_string(), self.version.to_json());
    d.insert("totalphys".to_string(), self.disk.to_json());
    Json::Object(d)
  }
}
