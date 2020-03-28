// 
fn exec<F>(f: F)
where F: Fn() {
    f();
}

fn feed3<F>(f: F) -> i32
where F: Fn(i32) -> i32 {
    f(3)
}

struct A<F: Fn(i32) -> i32> {
    f: F
}

// beware of the 'dyn' and 'move' keywords
fn create() -> Box<dyn Fn()> {
    Box::new(move || println!("This is a closure in a box!"))
}

pub fn run() {
    let f = |i| i + 1;
    let x = 10;
    println!("{}", f(x));

    let p = || println!("this is a closure without parameters");
    p();
    // ============
    let mut c = 0;

    let mut inc = || {
        c += 1;
        println!("incremented by 1: {}", c);
    };

    inc();
    inc();
    inc();
    // ============
    let p = || println!("hello from exec function!");
    exec(p);

    let x = |i| i * 10;
    println!("3 * 10 = {}", feed3(x));

    let a = A {
        f: x,
    };
    println!("{}", (a.f)(4));

    // ============
    let x = create();
    x();

    // ============
    let v = vec![1, 2, 3, 4];
    println!("v {}", v.iter().any(|&x| x != 2));

    for i in v.iter() {
        println!("{}", i);
    }
}