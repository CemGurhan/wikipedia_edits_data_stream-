use eventsource::reqwest::Client;
use reqwest::Url;

#[tokio::main]
async fn main() {
    let new_reqwest_client = reqwest::Client::new();
    let client = Client::new(Url::parse("https://stream.wikimedia.org/v2/stream/recentchange").unwrap());
    for event in client {
        let event_unwrapped = event.unwrap();
        let _result = new_reqwest_client.post("http://0.0.0.0:8080/production").body(event_unwrapped.data).send().await;
    }
}