//
macro_rules! a_macro {
    () => {
        println!("This is a macro");
    };
}

macro_rules! x_and_y {
    // we use {} here
    (x => $e:expr) => {
        println!("x: {}", $e)
    };
    // we use () here
    (y => $e:expr) => {
        println!("y: {}", $e)
    };
}

macro_rules! build_fn {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

macro_rules! print_ex {
    ($e:expr) => {
        println!("{:?} = {:?}", stringify!($e), $e)
    };
}

macro_rules! exame {
    ($l:expr; and $r:expr) => (
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($l),
            stringify!($r),
            $l && $r)
    );

    ($l:expr; or $r:expr) => (
        println!(
            "{:?} or {:?} is {:?}",
            stringify!($l),
            stringify!($r),
            $l || $r)
    );
}

macro_rules! compr {
    ($id1: ident | $id2: ident <- [$start: expr; $end: expr], $cond: expr) => {
        {
            let mut vec = Vec::new();

            for num in $start..$end + 1 {
                if $cond(num) {
                    vec.push(num);
                }
            }
            vec
        }
    };
}

fn even(x: i32) -> bool {
    x%2 == 0
}

fn odd(x: i32) -> bool {
    x%2 != 0
}

use std::collections::HashMap;
//  * here means: can repeat 0 or infinity times (>=0)
//  + means repeat once at least (>=1)
macro_rules! new_map {
    ($($key: expr => $val: expr)*) => {
    // another version with comma
    // ($($key: expr => $val: expr),*) => {
        {
            let mut map = HashMap::new();

            $(
                map.insert($key, $val);
            )*

            map
        }
    };
}

macro_rules! calc {
    (evalu $e:expr) => {{
        {
            let val: usize = $e;
            println!("{} = {}", stringify!($e), val);
        }
    }};

    (evalu $e:expr, $(evalu $es:expr), +) => {
        {
            calc! {evalu $e}
            calc! {$(evalu $es), +}
        }
    };
}

fn main() {
    a_macro!();

    x_and_y!(x => 10);
    x_and_y!(y => 10 + 20);

    build_fn!(hi_there);
    hi_there();

    print_ex!({
        let y = 20;
        let z = 30;
        z + y + 10 * 3 * 100
    });

    exame!(1 == 1; and 2 == 1 + 1);

    let evens = compr![x | x <- [1;10], even];
    print!("{:?}", evens);

    let odds = compr![y | y <- [1;10], odd];
    println!("{:?}", odds);

    let m = new_map!{
        "one" => 1
        "two" => 2
        "three" => 3
    };

    // let m = new_map!{
    //     "one" => 1,
    //     "two" => 2,
    //     "three" => 3
    // };

    println!("{:?}", m);

    calc! {
        evalu 4 * 5,
        evalu 4 + 10,
        evalu (10 * 3) - 20
    };
}
