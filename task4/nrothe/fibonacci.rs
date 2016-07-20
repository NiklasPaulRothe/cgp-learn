use std::iter::Iterator;

fn main() {
    for i in Fib::new().take(20) {
        println!("{}", i);
    }
}

struct Fib {
    current: u32,
    next: u32,
}

impl Fib {
    pub fn new() -> Fib {
        Fib {
            next: 1,
            current: 0,
        }
    }
}

impl Iterator for Fib {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let tmp = self.current;
        self.current = self.next;
        self.next = self.current + tmp;
        Some(self.current)
    }
}
