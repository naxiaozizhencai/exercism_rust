fn main() {
    println!("hello word")
}
pub fn square_of_sum(n: u32) -> u32 {
    let num = n * (n + 1) / 2;
    num.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..=n).map(|n| n.pow(2)).sum()
}
pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
