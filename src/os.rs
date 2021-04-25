use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::env::var;

pub fn get_os() -> String {
  let file = File::open("/etc/os-release").unwrap();
  let mut buf_reader = BufReader::new(file);
  
  // /etc/os-release should always be readable 
  let mut osr = String::new();
  buf_reader.read_to_string(&mut osr).unwrap();
  
  let id: String = osr
    .split('\n')
    .filter(|&s| s.contains("ID"))
    .take(1)
    .collect();

  let id_string = id
    .split("=")
    .nth(1)
    .unwrap()
    .replace("\"", "");

  id_string
}

pub fn get_kernel() -> String { 
  let file = File::open("/proc/sys/kernel/osrelease").unwrap();
  let mut buf_reader = BufReader::new(file);
  
  let mut kernel = String::new();
  buf_reader.read_line(&mut kernel).unwrap();
  kernel.pop();

  kernel
}

pub fn get_shell() -> String {
  let shell = var("SHELL").unwrap_or("Unknown".to_string());

  // will only work if $SHELL is /usr/bin/___
  // TODO: check for /bin/___ or plain ol' ___
  let shell_string = shell
    .split("/")
    .nth(3)
    .unwrap()
    .to_string();

  shell_string
}