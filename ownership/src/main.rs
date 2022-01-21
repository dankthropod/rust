// fn main() { // first_string is not declared yet -> has no value
//  let first_string = String::from("freeCodeCamp"); // first_string is now owner of the value "freeCodeCamp"
//  let second_string = first_string; // second_string takes ownership of the value "freeCodeCamp"

//  println!("Hello, {}!", first_string); // first_string is NOT valid, because the value was moved to second_string
//}

fn main() {
  let first_string: String = String::from("freeCodeCamp");
  let _second_string: &String = &first_string; // first_string is still the owner of the value "freeCodeCamp"

  println!("Hello, {}!", first_string);
}

// The ampersand (&) indicates that the value is a
// reference. That is, second_string no longer takes
// ownership of "freeCodeCamp", but, instead, points to
// the same point in memory as first_string.

// Also the variables that point to the same point in memory are preceeded with an underscore