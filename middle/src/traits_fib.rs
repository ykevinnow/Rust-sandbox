// 

struct Fib {
    c: u32,
    n: u32,
}

impl Iterator for Fib {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let n = self.c + self.n;
        self.c = self.n;
        self.n = n;

        Some(self.c)
    }
}

fn fib() ->Fib {
    Fib{c: 1, n: 1}
}

pub fn run() {
    // we got iterator trait already, so we can use 'take' and 'skip', etc
    for j in fib().take(10) {
        println!("{}", j);
    }

    for j in fib().skip(14).take(10) {
        println!("{}", j);
    }
}