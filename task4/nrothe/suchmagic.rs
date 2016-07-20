 use std::ops::{Mul, Add};

fn magic<T: Mul + Add + Copy>(par1: T, par2: T) -> (<T as Mul>::Output, <T as Add>::Output) {
    let p1 = par1;
    let p2 = par2;
    (par1 * par2, p1 + p2)
}

fn main() {
    println!("{:?}", magic(3, 4));
}
