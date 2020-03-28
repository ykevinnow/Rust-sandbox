// 
// ===============================
// use std::fmt;
// fn p<T: fmt::Debug>(x: T) {
//     println!("{:?}", x);
// }

// ===============================
// struct A<T1> {
//     x: T1,
// }

// impl <T2> A<T2> {
//     fn item(&self) ->&T2 {
//         &self.x
//     }
// }

// ===============================
// struct A<U, V> {
//     x: U,
//     y: V,
// }

// struct B<V> {
//     x: V,
//     y: V,
// }

// ===============================
use std::ops::Mul;

trait Shape<T> {
    fn area(&self) -> T;
}

struct Rectangle<T: Mul> {
    x: T,
    y: T,
}

impl <T: Copy> Shape<T> for Rectangle<T>
    where T: Mul<Output = T>, {
        fn area(&self) -> T {
            self.x * self.y
        }
}
// Or
// impl <T: Mul<Output = T> + Copy> Shape<T> for Rectangle<T> {
//     fn area(&self) -> T {
//         self.x * self.y
//     }
// }

// struct Circle<T: Mul> {
//     radius: T,
// }

// Error:TODO, we can not mul 3.14 directly
// impl <T: Mul<Output = T> + Copy> Shape<T> for Circle<T> {
//     fn area(&self) -> T {
//         3.14 * self.radius * self.radius
//     }
// }

// 13:01
pub fn run() {
    let r = Rectangle {x: 10, y: 20};
    let r2 = Rectangle {x: 10.10, y:20.31};

    // let c = Circle {radius: 12};
    // let c2 = Circle {radius: 12.12};

    println!("{} {}", r.area(), r2.area());
    // println!("{} {}", c.area(), c2.area());
}