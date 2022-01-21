fn main() {
  let my_str = "Hello, world!";
  let is_error = true;
  println!("{}", my_str);
  if is_error {
    panic!("There was an error!")
  }
}
