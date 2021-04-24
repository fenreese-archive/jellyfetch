use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[cfg(target_family = "unix")]
pub fn get_os() -> String {
  let file = File::open("/etc/os-release").unwrap();
  let mut buf_reader = BufReader::new(file);
  
  // /etc/os-release should always be readable 
  let mut osr = String::new();
  buf_reader.read_to_string(&mut osr).unwrap();
  
  let id: String = osr
    .split('\n')
    .filter(|&thing| thing.contains("ID"))
    .take(1)
    .collect();

  let id_string = id
    .split("=")
    .nth(1)
    .unwrap()
    .replace("\"", "");

  id_string
}