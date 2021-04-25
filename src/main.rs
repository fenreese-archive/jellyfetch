mod whoami;
mod os;

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
  println!("   .) (' ).('   ");
  println!("  '  ) (  ).    ");
  println!("   .'( .)'      ");
  println!("     .).'       ");
}