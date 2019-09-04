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
  // get the mac address
  // param eth: the eth name, like 'eth0'
  // return the mac address or empty
  pub fn get_mac_addr( eth: String) -> String {
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
          adptr.copy_from(ptr.offset(16) as *const i8, 16); // 16 mean ifreq::ifrn_name len
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

  // get ip and mac addr
  //
  //
  pub fn get_ip_addr() -> i32 {
    unsafe {
      let socket = libc::socket(libc::AF_INET, libc::SOCK_DGRAM, 0);
      if socket > 0 {
        struct IfConf {
          ifc_len :   i32,
          ifcu_req :  u32,
        }

        let mut buf = [0u8; 512];
        let ifconf = IfConf {
          ifc_len : 512,
          ifcu_req : *(buf.as_ptr() as *const u32),
        };

        let ptr = buf.as_mut_ptr();
        let err = libc::ioctl(socket, libc::SIOCGIFCONF, &ifconf);
        if 0 == err {
          struct InAddr {
            s_addr: u32,
          }
          // from libc import inet_ntoa
          extern {
            fn inet_ntoa(__in : InAddr) -> *mut c_char;
          }

          let count = ifconf.ifc_len/40; // 40 mean sizeof(struct ifreq)
          for i in 0..count {
            let eth_name = String::from_utf8_lossy(&buf[(i*40) as usize ..(i*40+16) as usize]);
            let mac_addr = ClientInfo::get_mac_addr(eth_name.to_string());

            let c : * const u32 = ptr.offset((20+i*40) as isize) as *const u32;
            let tmp = InAddr {
              s_addr : *c,
            };
            let b = inet_ntoa(tmp);
            let ip = CStr::from_ptr(b as *const c_char).to_string_lossy().to_string();

            println!("eth: {} IP:{:?} MAC:{}", eth_name, ip, mac_addr);
          }
        }
        libc::close(socket);
      }
    }
    0
  }

  //
  pub fn get_disk_size() -> i32 {
    0
  }

  //
  pub fn get_memory_size() -> i32 {
    0
  }

  //
  pub fn get_host_name() -> i32 {
    0
  }

  //
  pub fn get_current_username() -> i32 {
    0
  }

  //
  pub fn get_user_home() -> i32 {
    0
  }

  //
  pub fn get_product_info() -> i32 {
    0
  }

  //
  pub fn get_os_release() -> i32 {
    0
  }

  //
  pub fn get_issue() -> i32 {
    0
  }

  //
  pub fn get_redhat_release() -> i32 {
    0
  }

  //
  pub fn get_os_name() -> i32 {
    0
  }

  //
  pub fn availd_guid() -> bool {
    true
  }

  //
  pub fn get_bios_uuid() -> i32 {
    0
  }

  //
  pub fn get_disk_uuid() -> i32 {
    0
  }

}
