///
///
mod request;

use request::{ http_get };

use std::collections::HashMap;

trait Request {
  /// send the heartbeat to server, the interval is 300s
  fn heart_beat(&self);
  /// send the client info to server
  fn client_info();
  /// send the env platform info to server
  fn epinfo();
  /// request the authorization
  fn auth();
}

trait OnRequest {
    // add code here
}

#[derive(Default,Debug)]
pub struct Communicator {
  center : Option<String>,

}

impl Communicator {
  pub fn new() -> Self {
    Self {
      center : None,
    }
  }

  /// set the center address
  pub fn center(&mut self, url : &str) -> &mut Self {
    self.center = Some(url.to_string());
    self
  }
}



impl Request for Communicator {
  fn heart_beat(&self) {
    let a : HashMap<String, String> = HashMap::new();
    http_get("", a);
  }

  fn client_info() {

  }

  fn epinfo() {

  }

  fn auth() {

  }
}
