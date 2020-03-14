use influxdb2::Client;
use std::env;

#[tokio::main]
async fn main() {
    let host = env::var("INFLUX_HOST").unwrap_or("http://localhost:9999".into());
    let org = env::var("INFLUX_ORG").expect("Expected organization name!");
    let token = env::var("INFLUX_TOKEN").expect("Expected token");

    let client = Client::new(host, org, token).unwrap();

    let result = client.bucket_list_all().await.unwrap();
    println!("{:?}", result);
}
