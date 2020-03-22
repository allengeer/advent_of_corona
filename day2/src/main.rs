use factorial::Factorial;

fn de_bruijn(p: u32, n: u32) -> u128 {
    (((p * n) as u128).factorial() / ((n as u128).factorial()).pow(p) ) as u128
}

fn main() {
    println!("This is the De Bruijn Sequence Value B(3,10) = {}", de_bruijn(3,10));
    println!("This is the De Bruijn Sequence Value B(5,5) = {}", de_bruijn(5,5));
}
