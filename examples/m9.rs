#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}
const HOURS_PER_DAY: i32 = 24;
const MINUTES_PER_HOUR: i32 = 60;
const MINUTES_PER_DAY: i32 = MINUTES_PER_HOUR * HOURS_PER_DAY;
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = total_minutes(hours, minutes);
        let hours = total_minutes / MINUTES_PER_HOUR;
        let minutes = total_minutes % MINUTES_PER_HOUR;
        Self { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }

    pub fn to_string(&self) -> String {
        return format!("{:0>2}:{:0>2}", self.hours, self.minutes);
    }
}
fn total_minutes(hours: i32, minutes: i32) -> i32 {
    let total_minutes = (hours * MINUTES_PER_HOUR + minutes) % MINUTES_PER_DAY;
    if total_minutes >= 0 {
        total_minutes
    } else {
        total_minutes + MINUTES_PER_DAY
    }
}
fn main() {
    let res = Clock::new(0, 25).add_minutes(120).to_string();
    println!("{}", res);
}
