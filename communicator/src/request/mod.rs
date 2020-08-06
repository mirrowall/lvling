//!
//! http request, use libcurl
//!  
use std::collections::HashMap;
use std::io::Result;

extern crate curl;
use curl::easy::{Easy, List};

pub fn http_get(url: &str, header: HashMap<String, String>) -> Result<String> {
    let mut dst = Vec::new();

    let mut easy = Easy::new();
    easy.url(url).unwrap();

    let mut list = List::new();
    for (key, value) in header {
        list.append(format!("{}: {}", key, value).as_str()).unwrap();
    }
    easy.http_headers(list).unwrap();

    let mut transfer = easy.transfer();
    transfer.write_function(|new_data| {
        dst.extend_from_slice(new_data);
        Ok(new_data.len())
    }).unwrap();
    transfer.perform().unwrap();

    // Ok(String::from_utf8(dst).unwrap())
    println!("{:?}", dst);
    Ok("hello".to_string())
}
