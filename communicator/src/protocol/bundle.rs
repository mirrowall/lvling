//! this module define the server message struct

use std::collections::HashMap;


struct ContentPart<T> {
  values : HashMap<String, Vec<T>>,
}

struct MessagePart<T> {
  cmsgtype  : u32,
  content   : HashMap<String, T>,
}

struct MessageBundle<T> {
  msg : Vec<T>,
}

impl MessageBundle {
  pub fn serialize() -> String {

  }
}
