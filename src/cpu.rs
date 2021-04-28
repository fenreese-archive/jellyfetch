use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn get_cpu() -> String {
  let file = File::open("/proc/cpuinfo").unwrap();
  let mut buf_reader = BufReader::new(file);

  let mut cpu_info = String::new();
  buf_reader.read_to_string(&mut cpu_info).unwrap();

  // there are 6 instances of this for my cpu
  let cpu_name_string : String = cpu_info
    .split('\n')
    .filter(|&s| s.contains("model name"))
    .take(1)
    .collect();
  
  let cpu : String = cpu_name_string
    .split(':')
    .nth(1)
    .unwrap()
    .trim_matches(|c: char| c.is_whitespace())
    .to_string();
  
  // TODO: remove extra stuff from CPU name
  cpu
}