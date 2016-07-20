use std::ops::{Mul, Add};

fn magic<T: Mul + Add + Copy>(par1: T, par2: T) -> (<T as Mul>::Output, <T as Add>::Output) {
    (par1 * par2, par1 + par2)
}

fn main() {
    println!("{:?}", magic(3, 4));
}
