use chrono::{Local,Utc};

fn main() {
    let now=Utc::now();
    println!("Current time: {}", now);
    let formatted_time = now.format("%d-%m-%Y ").to_string();

    let local_time = Local::now();
    println!("Local time: {}", local_time);
    let formatted_local_time = local_time.format("%d-%m-%Y ").to_string();

}
