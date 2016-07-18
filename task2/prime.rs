/// Prints first 20 prime numbers
fn main() {
    let mut cnt = 0;
    let mut n = 0;
    while cnt < 20 {
        n += 1;
        if isprime(n){
            cnt += 1;
            println!("{}: {} is prime",cnt,n);
        }
    }
}
fn isprime(n: i32) -> bool {
    let mut isprime:bool = true;

    for i in 2..n/2 + 1 {
        if n % i == 0 {
            isprime = false
        }
    }
    isprime
}
