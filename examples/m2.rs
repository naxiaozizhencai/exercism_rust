pub fn production_rate_per_hour(speed: u8) -> f64 {
    let mut rate = 0.0;
    if speed >= 1 && speed <= 4 {
        rate = 1.0;
    }else if speed >= 5 && speed <= 8 {
        rate = 0.9;
    }else if speed >= 9 && speed <= 10 {
        rate = 0.77
    }

    return rate * (221 as f64) * speed as f64;
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    
    let res = production_rate_per_hour(speed) / 60 as f64;
    return res as u32 ;
}

fn main() {
    let number = 9;
    let res:f64 = number.into();
    println!("{:?}",  res * 2.3);
    println!("{:?}", production_rate_per_hour(6));
    println!("{:?}", working_items_per_minute(6));
}