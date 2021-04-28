mod whoami;
mod os;
mod memory;
mod cpu;
mod uptime;

//   "  .-;':':'-.    "
//   " {'.'.'.'.'.}   "
//   "  )        '`.  "
//   " '-. ._ ,_.-='  "
//   "  `). ( `);(    "
//   "  ('. .)(,'.)   "
//   "   .) (' ).('   "
//   "  '  ) (  ).    "
//   "   .'( .)'      "
//   "     .).'       "

fn main() {
  println!("  .-;':':'-.    ");
  println!(" {{'.'.'.'.'.}}   {}@{}", whoami::get_username(), whoami::get_hostname());
  let br = "-".repeat(whoami::get_username().len() + whoami::get_hostname().len() + 1);
  println!("  )        '`.  {}", br);
  println!(" '-. ._ ,_.-='  os\t{}", os::get_os());
  println!("  `). ( `);(    kernel\t{}", os::get_kernel());
  println!("  ('. .)(,'.)   shell\t{}", os::get_shell());
  println!("   .) (' ).('   memory\t{}", memory::get_memory());
  println!("  '  ) (  ).    cpu\t{}", cpu::get_cpu());
  println!("   .'( .)'      uptime\t{}", uptime::get_uptime());
  println!("     .).'       ");
}