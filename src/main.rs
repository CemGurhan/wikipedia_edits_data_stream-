use eventsource::reqwest::Client;
use reqwest::Url;
use tokio::task;

#[tokio::main]
async fn main() {
    for i in 0..5 {
        task::spawn_blocking(move || {
            println!("spawned sender thread: {}", i+1);
            let new_reqwest_client = reqwest::blocking::Client::new();
            let client = Client::new(Url::parse("https://stream.wikimedia.org/v2/stream/recentchange").unwrap());
            for event in client {
                let event_unwrapped = event.unwrap();
                let result = new_reqwest_client.post("http://0.0.0.0:8080/production").body(event_unwrapped.data).send();
                // println!("{:?}", result);
                // println!("{:?}", event.unwrap());
            }
        });
    }
}


async fn _sender() {
    for i in 0..5 {
        task::spawn(async move {
            println!("spawned sender thread: {}", i+1);
            let new_reqwest_client = reqwest::Client::new();
            let client = Client::new(Url::parse("https://stream.wikimedia.org/v2/stream/recentchange").unwrap());
            for event in client {
                let event_unwrapped = event.unwrap();
                let _result = new_reqwest_client.post("http://0.0.0.0:8080/production").body(event_unwrapped.data).send().await;
            }
        });
    }
}