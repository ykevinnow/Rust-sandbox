// a few definite values
// inner attributes are usually found at the beginning of source files
#![allow(dead_code)]
// ------------Version 0.01 Start-------------
enum Movement {
    // 
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement) {
    // Perform action depending on info
    match m {
        Movement::Up => println!("Avatar moving up"),
        Movement::Down => println!("Avatar moving down"),
        Movement::Left => println!("Avatar moving left"),
        Movement::Right => println!("Avatar moving right"),
    }
}

// ------------Version 0.01 End-------------------

// ------------Version0.02 Start------------------


#[derive(Debug)]
enum Direction {
    Up(Point),
    Down(Point),
    Left(Point),
    Right(Point),
}

#[derive(Debug)]
enum Keys {
    UpKey(String),
    DownKey(String),
    LeftKey(String),
    RightKey(String),
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Direction {
    fn match_direction(&self) -> Keys {
        match *self {
            Direction::Up(_) => Keys::UpKey(String::from("Pressed w")),
            Direction::Down(_) => Keys::DownKey(String::from("Pressed s")),
            Direction::Left(_) => Keys::LeftKey(String::from("Pressed a")),
            Direction::Right(_) => Keys::RightKey(String::from("Pressed d")),
        }
    }
}

impl Keys {
    fn destruct(&self) -> &String {
        match *self {
            Keys::UpKey(ref s) => s,
            Keys::DownKey(ref s) => s,
            Keys::LeftKey(ref s) => s,
            Keys::RightKey(ref s) => s,
        }
    }
}

enum Shape {
    Rectangle {width: u32, height: u32},
    Square(u32),
    Circle(f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match *self {
            Shape::Rectangle {width, height} => (width * height) as f64,
            Shape::Square(ref s) => (s * s) as f64,
            Shape::Circle(ref r) => 3.14 * (r * r),
        }
    }
}
// ------------Version0.02 End  ------------------
pub fn run() {
    // -------Version 0.01 Start -------------
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Right;
    let avatar4 = Movement::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
    // --------Version 0.02 End -----------
    let u = Direction::Up(Point {x: 0, y: 1});
    let k = u.match_direction();
    let x = k.destruct();

    println!("{}", x);

    // ---Shape--------
    let r = Shape::Rectangle{width: 10, height: 20};
    let s = Shape::Square(10);
    let c = Shape::Circle(5.0);

    let ar = r.area();
    println!("{}", ar);

    let aq = s.area();
    println!("{}", aq);

    let ac = c.area();
    println!("{}", ac);
}