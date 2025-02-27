use std::time::{Instant, SystemTime};
use chrono::{DateTime, Local, Utc};

fn main() {
    print_time();
    calculate_duration();
}

fn print_time() {
    let system_now = SystemTime::now();
    let datetime: DateTime<Utc> = system_now.into();

    println!("UTC time: {}", datetime.format("%Y-%m-%d %H:%M:%S"));

    let local_time: DateTime<Local> = SystemTime::now().into();
    println!("Local time: {}", local_time.format("%Y-%m-%d %H:%M:%S"));
    println!("Local time: {}", local_time.format("%Y-%mm-%d %H:%M:%S"));
}

fn calculate_duration() {
    let now = Instant::now();

    let mut res = 0;
    for _ in 1..50_000_000 {
        res = res + 1;
    }

    println!("{:?}", now.elapsed());
}
