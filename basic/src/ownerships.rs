// 

// return the ownship
fn re(v: Vec<i32>) -> Vec<i32> {
    println!("{}", v[121] + v[111]);
    v
}

// type 1 borrow
fn borrow1(v: &Vec<i32>) {
    println!("{}", (*v)[10] + (*v)[12]);
}

// type 2 borrow
fn borrow2(v: &Vec<i32>) {
    println!("{}", v[10] + v[11]);
}

pub fn run() {
    let mut v = Vec::new();

    for i in 0..999 {
        v.push(i);
    }

    v = re(v);
    println!("Still own v: {} {}", v[0], v[1]);

    borrow1(&v);
    println!("Still own v: {} {}", v[0], v[1]);

    borrow2(&v);
    println!("Still own v: {} {}", v[0], v[1]);

    // ref keyword
    let u = 10;
    let v = &u;
    let ref z = u;

    // if v == u {
    //     println!("v equals u.");
    // }

    if z == v {
        println!("z equals v.");
    }
}