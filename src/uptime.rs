use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn get_uptime() -> String {
  let file = File::open("/proc/uptime").unwrap();
  let mut buf_reader = BufReader::new(file);

  let mut uptime_info = String::new();
  buf_reader.read_to_string(&mut uptime_info).unwrap();

  let uptime = uptime_info
    .split_whitespace()
    .nth(0)
    .unwrap()
    .to_string()
    .parse::<f32>()
    .unwrap();
  
  let days = (uptime) as i32 / 60 / 60 / 24;
  let hours = (uptime) as i32 / 60 / 60 % 24;
  let min = (uptime) as i32 / 60  % 60;

  let mut up = "".to_string();

  if days > 0 {
    up.push_str(&(format!("{}d ", days)));
  }
  if hours > 0 {
    up.push_str(&(format!("{}h ", hours)));
  }

  up.push_str(&(format!("{}m", min)));

  up
}