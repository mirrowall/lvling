//! copyrigth
//! get client info
//!
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::cell::RefCell;
use std::mem;
use libc;
use std::ffi::CStr;

pub struct ClientInfo;

impl ClientInfo {
  pub fn get_mac_addr( eth: &String) -> String {
    let mut ret : String = String::new();
    unsafe {
      let socket = libc::socket(libc::AF_INET, libc::SOCK_DGRAM, 0);
      if socket > 0 {
        let mut buf = [0u8; 40];
        let ptr = buf.as_mut_ptr();
        ptr.copy_from(eth.as_ptr(), eth.len());
        let servaddr = libc::ioctl(socket, libc::SIOCGIFHWADDR, ptr);
        if servaddr >= 0 {
          let ad : libc::sockaddr = std::mem::zeroed();
          let adptr = &ad as *const libc::sockaddr as * mut libc::sockaddr as * mut c_char;
          adptr.copy_from(ptr.offset(16) as *const i8, 16);
          for i in 0..6 {
            ret.push_str(&(format!("{:x}:", ad.sa_data[i])));
          }
          ret.pop();
        }
        libc::close(socket);
      }
    }
    ret
  }
}
