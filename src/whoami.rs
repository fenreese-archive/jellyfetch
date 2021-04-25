use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::env::var;

pub fn get_username() -> String {
  var("USER").unwrap_or("Unknown".to_string())
}

pub fn get_hostname() -> String {
  // hostname comes from /etc/hostname
  // TODO: not assume hostname is just 1 line
  let file = File::open("/etc/hostname").unwrap();
  let mut buf_reader = BufReader::new(file);
  
  let mut host = String::new();
  buf_reader.read_line(&mut host).unwrap();
  host.pop(); // reading files leaves you with a trailing nl

  host
}


