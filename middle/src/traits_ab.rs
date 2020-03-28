// use std::ops;

struct A {
    a: String,
}

// struct A;
// struct B;
// #[derive(Debug)]
// struct AB;
// #[derive(Debug)]
// struct BA;

// impl ops::Add<B> for A {
//     type Output = AB;

//     fn add(self, _rhs: B) -> AB {
//         AB
//     }
// }

impl Drop for A {
    fn drop(&mut self) {
        println!("dropped {}", self.a);
    }
}

// impl ops::Add<A> for B {
//     type Output = BA;

//     fn add(self, _rhs: A) -> BA {
//         BA
//     }
// }

pub fn run() {
    // for Add trait
    // println!("{:?}", A + B);
    // println!("{:?}", B + A);
    // for Drop trait
    let _a = A {a: String::from("A")};
    {
        let _b = A {a: String::from("B")};
        {
            let _c = A {a: String::from("C")};

            println!("leaving inner scope 2");
        }
        println!("leaving inner scope 1");
    }
    println!("program ending");
}