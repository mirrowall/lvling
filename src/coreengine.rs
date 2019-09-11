//!
//! 此模块应该是封装EPOLL, 用EPOLL机制来对NOTIFY进行截控,现使用RUST官方提供的MIO
//!
use mio::*;
use std::io;
use std::os::raw::c_char;
use std::collections::HashMap;

// fro inotify
use std::os::unix::io::RawFd;
use std::os::unix::io::AsRawFd;
use mio::unix::EventedFd;

extern {
  // extern int inotify_init (void) __THROW;
  fn inotify_init() -> i32;
  // extern int inotify_add_watch (int __fd, const char *__name, uint32_t __mask)
  fn inotify_add_watch(__fd : i32, __name : *const c_char, __mask : u32) -> i32;
  // extern int inotify_rm_watch (int __fd, int __wd) __THROW;
  fn inotify_rm_watch(__fd : i32, __wd: i32) -> i32;
}

const NOTIFY_EVENT : Token = Token(0);

/// from libc
struct NotifyEvnet {
  wd : i32,
  mask : u32,
  cookie : u32,
  __flexarr : *const c_char,
}

struct INotify {
  fd : RawFd,
}

impl INotify {
  fn new() -> io::Result<INotify> {
    Ok (
      Self {
        fd: unsafe { inotify_init() }
      }
    )
  }

  fn file_notify_add_watch(&mut self, path : &String) -> i32 {
    unsafe {
      return inotify_add_watch(self.fd, path.as_ptr() as *const c_char, libc::IN_ALL_EVENTS);
    }
  }

  fn file_notify_rm_watch(__fd : i32, __wd: i32) -> i32 {
    unsafe {
      return inotify_rm_watch(__fd, __wd);
    }
  }
}

impl Evented for INotify {
  fn register(&self, poll: &Poll, token: Token,
                interest: Ready, opts: PollOpt) -> io::Result<()> {
    EventedFd(&self.fd).register(poll, token, interest, opts)
  }

  fn reregister(&self, poll: &Poll, token: Token,
                interest: Ready, opts: PollOpt) -> io::Result<()> {
    EventedFd(&self.fd).reregister(poll, token, interest, opts)
  }

  fn deregister(&self, poll: &Poll) -> io::Result<()> {
    EventedFd(&self.fd).deregister(poll)
  }
}

impl AsRawFd for INotify {
  fn as_raw_fd(&self) -> RawFd {
    return self.fd;
  }
}

// inotify end

pub struct CoreEngine {
  poll   : Box<Poll>,
  notify : INotify,

  fds : Box<HashMap<i32, String>>,
}

impl CoreEngine {
  pub fn new() -> Self {
    Self {
      poll : Box::new(Poll::new().unwrap()),
      notify : INotify::new().unwrap(),
      fds : Box::new(HashMap::new()),
    }
  }

  pub fn add_watch_folder(&mut self, path:&str) -> io::Result<i32> {
    let wd = self.notify.file_notify_add_watch(&path.to_string());
    if wd > 0 {
      self.fds.as_mut().insert(wd, path.to_string());
    }
    Ok(wd)
  }

  fn remove_watch_folder(&mut self, path:&String) -> io::Result<i32> {
    Ok(0)
  }

  fn get_notify_file(&self) -> io::Result<i32> {
    let mut buf = [0u8; 1024usize];
    unsafe {
      let event : NotifyEvnet = std::mem::zeroed();
    }
    Ok(0)
  }

  pub fn start_core_engine(&mut self) -> io::Result<i32> {
    // add the notify fd to epoll
    let err = self.poll.register(&self.notify,
                          NOTIFY_EVENT,
                          Ready::readable(),
                          PollOpt::edge());

    let mut events = Events::with_capacity(1024);
    loop {
      self.poll.poll(&mut events, None).unwrap();
      for event in events.iter() {
        match event.token() {
          NOTIFY_EVENT => {
            self.get_notify_file();
          }
          _ => {
            println!("fuck you");
          },
        }
      }
    }
  Ok(0)
  }
}

