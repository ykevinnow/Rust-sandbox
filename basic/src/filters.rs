// Closure, filter

// &&x
// no implementation for &i32 == i32
fn count(v: &Vec<i32>, val: i32) -> usize {
    v.into_iter().filter(|&&x| x == val).count()
}

pub fn run() {
    let v = vec![3, 3, 6, 1, 6, 7, 2, 4, 7, 8, 7, 4, 2, 5, 9, 0];
    for &i in &v {
        let r = count(&v, i);
        println!("{} is repeated {} times", i, r);
    }
    // for ownership
    println!("{}", v[1]);
}
