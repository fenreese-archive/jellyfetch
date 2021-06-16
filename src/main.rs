use colored::*;

mod whoami;
mod os;
mod memory;
mod cpu;
mod uptime;

const JELLY: &[&str] = &[
  "  .-;':':'-.    ",
  " {'.'.'.'.'.}   ",
  "  )        '`.  ",
  " '-. ._ ,_.-='  ",
  "  `). ( `);(    ",
  "  ('. .)(,'.)   ",
  "   .) (' ).('   ",
  "  '  ) (  ).    ",
  "   .'( .)'      ",
  "     .).'       ",
];

fn render(info: &[String]) {
  for (jelly_line, info_line) in JELLY.iter().zip(info) {
      println!("{}   {}", jelly_line.bright_cyan(), info_line);
  }
}

fn main() {
  let user_info = format!("{}{}{}", whoami::get_username().purple().bold(), "@", whoami::get_hostname().purple().bold());
  let os_info = format!("{}  {}", "os".red(), os::get_os());
  let kernel_info = format!("{}  {}", "kernel".yellow(), os::get_kernel());
  let shell_info = format!("{}  {}", "shell".green(), os::get_shell());
  let memory_info = format!("{}  {}", "memory".blue(), memory::get_memory());
  let cpu_info = format!("{}  {}", "cpu".cyan(), cpu::get_cpu());
  let uptime_info = format!("{}  {}", "uptime".purple(), uptime::get_uptime());

  // colours
  let bright = format!(
    "{}{}{}{}{}{}{}{}",
    "██".bright_red(),
    "██".bright_yellow(),
    "██".bright_green(),
    "██".bright_cyan(),
    "██".bright_blue(),
    "██".bright_magenta(),
    "██".bright_black(),
    "██".bright_white()
  );
  let dark = format!(
    "{}{}{}{}{}{}{}{}",
    "██".red(),
    "██".yellow(),
    "██".green(),
    "██".cyan(),
    "██".blue(),
    "██".magenta(),
    "██".black(),
    "██".white()
  );

  render(&[
    user_info,
    os_info,
    kernel_info,
    shell_info,
    memory_info,
    cpu_info,
    uptime_info,
    "".to_string(),
    bright,
    dark
  ]);
}