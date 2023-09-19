use std::fs::File;
use std::io;
use std::{thread, time::Duration};

#[tokio::main]
#[allow(unused_must_use)]
#[allow(unused_mut)]
pub async fn main() {
    let mut webhook = String::new();
    let mut times = String::new();
    let mut delay = String::new();

    println!("\nWebhook:");
    io::stdin()
        .read_line(&mut webhook)
        .expect("uhhhhhh");

    println!("\nTimes to spam:");
    io::stdin()
        .read_line(&mut times)
        .expect("uhhhhhh");
    let timeint: i32 = times
        .trim()
        .parse::<i32>()
        .expect("uhhhhhh");

    println!("\nDelay:");
    io::stdin()
        .read_line(&mut delay)
        .expect("uhhhhhh");
    let delayint: u64 = delay
        .trim()
        .parse::<u64>()
        .expect("uhhhhhh");

    let mut file = File::open("hookconfig.json").unwrap();
    let json: serde_json::Value = serde_json::from_reader(file).expect("uhhhhhh");

    let client = reqwest::Client::new();

    println!();
    for i in 0..timeint {
        print!("Sending: {} | ", i);
        let resu = client.post(&webhook).json(&json).send().await.expect("send");
        match resu.status() {
            reqwest::StatusCode::NO_CONTENT => {
                println!("Success");
            },
            reqwest::StatusCode::TOO_MANY_REQUESTS => {
                println!("Ratelimited");
            },
            reqwest::StatusCode::NOT_FOUND => {
                panic!("Webhook Gone");
            },
            _ => {
                println!("something wrong");
            },
        };
        thread::sleep(Duration::from_millis(delayint));
    }
    println!("\nFinished")
}
