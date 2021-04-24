mod whoami;
mod os;

//   "  .-;':':'-.    "
//   " {'.'.'.'.'.}   "
//   "  )        '`.  "
//   " '-. ._ ,_.-='  "
//   "  `). ( `);(    "
//   "  ('. .)(,'.)   "
//   "   ) ( ,').(    "
//   "  ( .').'(').   "
//   "   .) (' ).('   "
//   "  '  ) (  ).    "
//   "   .'( .)'      "
//   "     .).'       "

fn main() {
  println!("  .-;':':'-.    ");
  print!(" {{'.'.'.'.'.}}   \t{}@{}", whoami::get_username(), whoami::get_hostname());
  let br = "-".repeat(whoami::get_username().len() + whoami::get_hostname().len() + 1);
  println!("  )        '`.  \t{}", br);
  println!(" '-. ._ ,_.-='  \tos\t{}", os::get_os());
  println!("  `). ( `);(    ");
  println!("  ('. .)(,'.)   ");
  println!("   ) ( ,').(    ");
  println!("  ( .').'(').   ");
  println!("   .) (' ).('   ");
  println!("  '  ) (  ).    ");
  println!("   .'( .)'      ");
  println!("     .).'       ");
}