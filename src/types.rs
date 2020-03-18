/*
Intergers
Floats
Boolean
Char
Tuples
Arrays
*/

// Rust is a statically typed language, but the compiler can usually infer what type we want to use based on the value
pub fn run() {
  // Default is "i32"
  let x = 1;

  // Default is "f64"
  let y = 2.5;

  // Add explicit type 
  let z: i64 = 234153456;

  // Find max size
  println!("Max i32: {}", std::i32::MAX);
  println!("MAX i63: {}", std::i64::MAX);

  //Boolean
  let is_active: bool = true;

  // Get boolean from expression
  let is_greater: bool = 10 > 5;

  let a1 = 'a';
  let face = '\u{1F600}'; // emoji

  println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}