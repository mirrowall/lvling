//! copyrigth
//! get client info
//!
use std::os::raw::c_char;
use std::mem;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use libc;
use std::ffi::CStr;
use std::collections::HashMap;
use std::io::prelude::*;
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
  pub fn get_ip_addr(ipaddr :& mut HashMap<String, (String, String)>) -> Result<i32, &str> {
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

            ipaddr.insert(eth_name.to_string(), (ip, mac_addr));
          }
        }
        libc::close(socket);
      }
    }
    Ok(0)
  }

  // get the disk size
  // as GB
  pub fn get_disk_size() -> u64 {
    unsafe {
      let mut stats : libc::statfs = mem::zeroed();
      let err = libc::statfs("/".to_string().as_ptr() as *const i8, &mut stats as *mut libc::statfs);
      if 0 == err {
        let total = stats.f_blocks as u64 * stats.f_bsize as u64;
        return total >> 30;
      }
    }
    0
  }

  // get the memory size
  // as MB
  pub fn get_memory_size() -> u64 {
    unsafe {
      let mut sysinfo : libc::sysinfo = mem::zeroed();
      let err = libc::sysinfo(&mut sysinfo as *mut libc::sysinfo);
      if err == 0 {
        return sysinfo.totalram/1024/1024;
      }
    }
    0
  }

  // get the host name
  pub fn get_host_name() -> String {
    unsafe {
      let mut uts : libc::utsname = mem::zeroed();
      if libc::uname(&mut uts as *mut libc::utsname) >= 0{
        return CStr::from_ptr(&uts.nodename as *const c_char).to_string_lossy().to_string();
      }
    }
    "".to_string()
  }

  //
  pub fn get_current_username(uid:u32) -> String {
    unsafe {
      let mut pass : libc::passwd = mem::zeroed();
      let mut buf = [0i8; 512];
      let mut result : *mut libc::passwd = mem::zeroed();

      if 0 == libc::getpwuid_r(uid, &mut pass as *mut libc::passwd, buf.as_mut_ptr(), 512, &mut result) {
        return CStr::from_ptr(pass.pw_name).to_string_lossy().to_string();
      }
    }
    "".to_string()
  }

  //
  pub fn get_user_home() -> String {
    unsafe {
      let user = ClientInfo::get_current_username(libc::getuid());
      if "root" == user {
        return "/root".to_string();
      }
      return format!("/home/{}", user);
    }
  }

  //
  pub fn get_product_info() -> std::io::Result<String> {
    let file = File::open("/etc/.productinfo")?;
    let buf_reader = BufReader::new(file);

    let mut product = String::new();
    for line in buf_reader.lines() {
      let tmp = &line.unwrap();
      if tmp.starts_with("-") {
        product += tmp.trim_left_matches("-");
        product += "-";
      }
    }
    product.truncate(product.len()-1);
    return Ok(product)
  }

  //
  pub fn get_os_release() -> std::io::Result<String> {
    let file = File::open("/etc/os-release")?;
    let reader = BufReader::new(file);

    let mut product = String::new();
    for line in reader.lines() {
      let d = &line.unwrap();
      if d.starts_with("NAME=") {
        product += d[5..].to_string().trim_matches('"');
      } else if d.starts_with("VERSION=") {
        product += d[8..].to_string().trim_matches('"');
      }
    }
    Ok(product)
  }

  //
  pub fn get_issue() -> std::io::Result<String> {
    let file = File::open("/etc/issue")?;
    let mut reader = BufReader::new(file);

    let mut issue = Vec::<u8>::new();
    reader.read_until(b'\n', &mut issue);

    Ok(String::from_utf8(issue).unwrap())
  }

  //
  pub fn get_redhat_release() -> std::io::Result<String> {
    let file = File::open("/etc/redhat-release")?;
    let mut reader = BufReader::new(file);

    let mut issue = Vec::<u8>::new();
    reader.read_until(b'\n', &mut issue);

    Ok(String::from_utf8(issue).unwrap())
  }

  //
  pub fn get_os_name() -> String {
    if Path::new("/etc/.productinfo").exists() {
      return ClientInfo::get_product_info().unwrap();
    } else if Path::new("/etc/os-release").exists() {
      return ClientInfo::get_os_release().unwrap();
    } else if Path::new("/etc/redhat-release").exists() {
      return ClientInfo::get_redhat_release().unwrap();
    } else if Path::new("/etc/issue").exists() {
      return ClientInfo::get_issue().unwrap();
    }

    "Linux".to_string()
  }

  //
  pub fn availd_guid(guid:&String) -> bool {
    let segs = guid.split('-');
    for s in segs {
      let bs = s.as_bytes();

    }
    true
  }

  //
  pub fn get_bios_uuid() -> String {
    if Path::new("/usr/sbin/dmidecode").exists() {
      unsafe {
        let file = libc::popen("/usr/sbin/dmidecode -s system-uuid|grep -v \"#\"".as_ptr() as *const c_char, "r".as_ptr() as *const c_char);
        let mut buf = [0u8; 128];
        libc::fgets(buf.as_mut_ptr() as *mut c_char, 128, file);

        return String::from_utf8_lossy(&buf).to_string();
      }
    }
    "".to_string()
  }

  //
  pub fn get_disk_uuid() -> String {
    if Path::new("/sbin/blkid").exists() {
      unsafe {
        let file = libc::popen("/sbin/blkid|grep -i uuid|head -n 1|cut -d \\\" -f 2".as_ptr() as *const c_char, "r".as_ptr() as *const c_char);
        let mut buf = [0u8; 128];
        libc::fgets(buf.as_mut_ptr() as *mut c_char, 128, file);

        return String::from_utf8_lossy(&buf).to_string();
      }
    }
    "".to_string()
  }

}
