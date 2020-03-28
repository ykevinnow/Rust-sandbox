// Used to check the condition of something and act on the result

pub fn run() {
  //
  let age: u8 = 30;
  let check_id: bool = true;
  let knows_person_of_age = true;

  // IF/ELSE
  if age >= 21 && check_id || knows_person_of_age {
      println!("Bartender: What would you like to drink?");
  } else if age < 21 && check_id {
      println!("Bartender: Sorry, you have to leave");
  } else {
      println!("Bartender: I'll need to see your ID");
  }

  // Shorthand if
  let is_of_age = if age >= 21 { true } else {false};
  println!("Is Of Age: {}", is_of_age);


  let n = 19;
  match n {
      1 => println!("One!"),
      2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
      13..=19 => println!("A teen"),
      _ => println!("Ain't special"),
  }

  let pair = (0, -2);
  match pair {
      (0, y) => println!("y: {}", y),
      (x, 0) => println!("x: {}", x),
      _      => println!("No match!")
  }

  let new_pair = (5, -5);
  match new_pair {
      (x, y) if x == y => println!("Equal"),
      (x, y) if x + y == 0 => println!("Equal Zero"),
      (x, _) if x % 2 == 0 => println!("X is even"),
      _ => println!("No match"),
  }

  let q = 5;
  match q {
      n @ 1 ..= 12 => println!("First part n: {}", n),
      n @ 13 ..= 19 => println!("Second part n: {}", n),
      _ => println!("No match")
  }
}