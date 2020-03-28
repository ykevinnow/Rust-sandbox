// 

// 
use List::Cons;
use List::End;

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    End,
}

pub fn run() {
    let b = Box::new(10);
    println!("b = {}", b);

    let y = 4;
    let x = &y;
    let z = Box::new(y);

    if *x == *z {
        println!("true");
    }

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(End))))));
    println!("{:?}", list);
}