pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    (dividend / divisor, dividend % divisor)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    return iter.enumerate().filter(|(k, _)| k % 2 == 0).map(|(_, v)| v);
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        self.0.abs() + self.1.abs()
    }
}
fn main() {
    let (a, b) = divmod(10, 3);
    println!("{:?},{:?}", a, b);
    println!("{:?}", Position(-333, 4).manhattan());
    let mut even_ints = evens(1_i16..);
    assert_eq!(even_ints.next(), Some(1));
}
