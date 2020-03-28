// Vectors - Resizable arrays
#![allow(dead_code)]

use std::mem;

pub fn run() {
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

      // Re-assign vlaue
  numbers[2] = 22;

  // Add on to vector
  numbers.push(5);
  numbers.push(6);

  // Pop off last value
  numbers.pop();

  println!("{:?}", numbers);
  // Get single value
  println!("Single value: {}", numbers[3]);

  // Get array length
  println!("Vector length: {}", numbers.len());

  // Array are stack allocated
  //println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));
  println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

  // Get slice
  let slice: &[i32] = &numbers[1..4];
  println!("Slice: {:?}", slice);

  // Loop through vector values
  for x in numbers.iter() {
      println!("Number: {}", x);
  }

  // Loop & mutate values
  for x in numbers.iter_mut() {
      *x *= 2;
  }

  println!("Numbers Vec: {:?}", numbers);

  // Version 0.02
  #[derive(Debug)]
  enum Sexual {
      Male,
      Female,
  }
  #[derive(Debug)]
  enum Person {
      Name(String),
      Age(u8),
      Sex(Sexual),
  }

  let connie = vec![
      Person::Name(String::from("Yangtianqi")),
      Person::Age(32),
      Person::Sex(Sexual::Female),
  ];
  println!("{:?}", connie);
}