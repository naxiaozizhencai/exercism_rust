fn main() {
    is_armstrong_number(3_999_999_999);
}
pub fn is_armstrong_number(num: u32) -> bool {
    let num_string = num.to_string();
    let count = num_string
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(num_string.len() as u32))
        .collect::<Vec<u32>>();
    // println!("{:?}", count);
    let mut sum: u32 = 0;
    for x in &count {
        sum = sum.saturating_add(*x)
    }
    // println!("{:?}", sum);
    if num == sum {
        return true;
    } else {
        return false;
    }
}
