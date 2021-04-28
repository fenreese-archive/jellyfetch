use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn kb_to_mb(val: String) -> u32 {
  let kb = val
    .split(":")
    .nth(1)
    .unwrap()
    .trim_matches(|c: char| c =='k' || c == 'B' || c.is_whitespace())
    .parse::<u32>()
    .unwrap();

  let mb = kb / 1024;

  mb
}

pub fn get_memory() -> String {
  let file = File::open("/proc/meminfo").unwrap();
  let mut buf_reader = BufReader::new(file);

  let mut mem_info = String::new();
  buf_reader.read_to_string(&mut mem_info).unwrap();

  let mem_total_string : String = mem_info
    .split('\n')
    .filter(|&s| s.contains("MemTotal"))
    .take(1)
    .collect();

  let mem_avail_string : String = mem_info
    .split('\n')
    .filter(|&s| s.contains("MemAvailable"))
    .take(1)
    .collect();

  let mem_total = kb_to_mb(mem_total_string);
  let mem_avail = kb_to_mb(mem_avail_string);
  
  format!("{}M / {}M", mem_total - mem_avail, mem_total)
}
