//!
//!
//!
mod clientinfo;
use clientinfo::ClientInfo;
use std::collections::HashMap;

mod coreengine;
use coreengine::CoreEngine;

use std::os::raw::c_char;
use std::ffi::CStr;
use std::mem;
use std::thread;
use std::time::Duration;

extern {
  fn gethostname(name:*mut c_char, len:usize) -> i32;
}

extern {
  // extern int inotify_init (void) __THROW;
  fn inotify_init() -> i32;
  // extern int inotify_add_watch (int __fd, const char *__name, uint32_t __mask)
  fn inotify_add_watch(__fd : i32, __name : *const c_char, __mask : u32) -> i32;
  // extern int inotify_rm_watch (int __fd, int __wd) __THROW;
  fn inotify_rm_watch(__fd : i32, __wd: i32) -> i32;
}



fn main() {
  // let mut ipaddr = HashMap::new();
  // let a = ClientInfo::get_ip_addr(&mut ipaddr);
  // match a {
  //   Ok(a) => {
  //     println!("{:?}", Some(a));
  //     for (k, v) in & ipaddr {
  //       println!("{}:{:?}", k, v);
  //     }
  //   }
  //   _ => {}
  // }

  // let b = ClientInfo::get_disk_size();
  // println!("{} GB", b);

  // let c = ClientInfo::get_memory_size();
  // println!("memory is {} MB", c);

  // let d = ClientInfo::get_host_name();
  // println!("host name is {}", d);

  // let e = ClientInfo::get_current_username(0);
  // println!("current user is {}", e);

  // let f = ClientInfo::get_user_home();
  // println!("user home path is {}", f);

  // let g = ClientInfo::get_product_info();
  // println!("product is {}", g.unwrap());

  // let h = ClientInfo::get_os_release();
  // println!("release is {}", h.unwrap());

  // let i = ClientInfo::get_issue();
  // println!("issue is {}", i.unwrap());

  // let j = ClientInfo::get_os_name();
  // println!("os name is {}", j);

  // let k = ClientInfo::get_bios_uuid();
  // println!("bios is {}", k);

  let mut engine = CoreEngine::new();
  engine.add_watch_folder("/home/miwoo/workspace/test");
  engine.start_core_engine();


}
