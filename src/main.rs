use eventsource::reqwest::Client;
use reqwest::Url;

fn main() {
    let client = Client::new(Url::parse("https://stream.wikimedia.org/v2/stream/recentchange").unwrap());
    for event in client {
        println!("{}", event.unwrap());
    }
}