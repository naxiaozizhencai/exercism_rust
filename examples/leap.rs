fn main() {
    let res = is_leap_year(1970);
    println!("{:?}", res)
}

pub fn is_leap_year(year: u64) -> bool {
    (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0)
}
