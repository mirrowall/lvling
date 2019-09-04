//!
//!
//!
mod clientinfo;
use clientinfo::ClientInfo;
// use std::cell::RefCell;
use std::collections::HashMap;

use std::os::raw::c_char;
use std::ffi::CStr;
use std::mem;

extern {
  fn gethostname(name:*mut c_char, len:usize) -> i32;
}

fn main() {
  let mut ipaddr = HashMap::new();
  let a = ClientInfo::get_ip_addr(&mut ipaddr);
  match a {
    Ok(a) => {
      println!("{:?}", Some(a));
      for (k, v) in & ipaddr {
        println!("{}:{:?}", k, v);
      }
    }
    _ => {}
  }

  let b = ClientInfo::get_disk_size();
  println!("{} GB", b);

  let c = ClientInfo::get_memory_size();
  println!("memory is {} MB", c);

  let d = ClientInfo::get_host_name();
  println!("host name is {}", d);
}
