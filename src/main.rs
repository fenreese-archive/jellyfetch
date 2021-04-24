mod whoami;

fn main() {
  println!("{}{}{}", whoami::get_username(), "@", whoami::get_hostname());
}