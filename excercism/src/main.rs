
use std::time::Duration;
use tokio::time::sleep;

async fn say_hello() {
	sleep(Duration::from_secs(2)).await;
	println!("Hello, world!");
}

#[tokio::main]
async fn main() {
	say_hello().await;
}