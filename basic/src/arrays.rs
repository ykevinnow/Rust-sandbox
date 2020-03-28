// Fixed list where elements are the same data types
use std::mem;

pub fn run() {
  let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

  // Re-assign vlaue
  numbers[2] = 22;

  println!("{:?}", numbers);
  // Get single value
  println!("Single value: {}", numbers[3]);

  // Get array length
  println!("Array length: {}", numbers.len());

  // Array are stack allocated
  //println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));
  println!("Array occupies {} bytes", mem::size_of_val(&numbers));

  // Get slice
  let slice: &[i32] = &numbers[1..4];
  println!("Slice: {:?}", slice);
}