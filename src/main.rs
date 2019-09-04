//!
//!
//!
mod clientinfo;
use clientinfo::ClientInfo;
// use std::cell::RefCell;


use std::os::raw::c_char;
use std::ffi::CStr;
use std::mem;

extern {
  fn gethostname(name:*mut c_char, len:usize) -> i32;
}

union Ta {
  haha : i32,
}

union Tb {
  hehe : i32,
}
struct tf {
  ha : Ta,
  he : Tb,
}

fn main() {
  let a = ClientInfo::get_ip_addr();
  println!("{}", a);
}
