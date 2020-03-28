// 

// For Display trait
use std::fmt;

// needed for {:?}
#[derive(Debug)]
struct Object {
    width: u32,
    height: u32,
}

impl Object {
    fn new(width: u32, height: u32) -> Object {
        Object {
            width,
            height,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn show(&self) {
        println!("{}x{} is {}", self.width, self.height, self.area());
    }
}

// Display trait for custom struct
impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Object Display format: ({}, {})", self.width, self.height)
    }
}
pub fn run() {
    let obj = Object::new(12, 18);
    obj.show();

    println!("{}", obj);
}