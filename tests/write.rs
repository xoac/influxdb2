use futures::stream::{StreamExt, TryStreamExt};
use influxdb2::{write::WriteQuery, Client, Precision};
use influxdb_line_protocol::*;
use std::env;

#[tokio::test]
async fn single_point_write() {
    let host = env::var("INFLUX_HOST").unwrap_or("http://localhost:9999".into());
    let org = env::var("INFLUX_ORG").expect("INFLUX_ORG env variable must be set");
    let token = env::var("INFLUX_TOKEN").expect("INFLUX_TOKEN env variable must be set");
    let bucket = env::var("INFLUX_BUCKET").expect("INFLUX_BUCKET env variable must be set");

    let client = Client::new(host, org.clone(), token).unwrap();

    let field = Field::new("filedKey", "fieldValue").unwrap();
    let point = Point::builder("test")
        .unwrap()
        .add_field(field)
        .build()
        .unwrap();

    let query = WriteQuery::with_org(&bucket, &org);
    let result = client.write(point, query).await.unwrap();
    println!("{:?}", result);
}

#[tokio::test]
async fn write_stream_of_points() {
    let host = env::var("INFLUX_HOST").unwrap_or("http://localhost:9999".into());
    let org = env::var("INFLUX_ORG").expect("INFLUX_ORG env variable must be set");
    let token = env::var("INFLUX_TOKEN").expect("INFLUX_TOKEN env variable must be set");
    let bucket = env::var("INFLUX_BUCKET").expect("INFLUX_BUCKET env variable must be set");

    let client = Client::new(host, org.clone(), token).unwrap();

    let field = Field::new("filedKey", "fieldValue").unwrap();
    let point = Point::builder("test")
        .unwrap()
        .add_field(field)
        .build()
        .unwrap();

    let query = WriteQuery::with_org(bucket, org).precision(Precision::Milli);

    let five = futures::stream::repeat(point).take(5);

    let worker = five
        .chunks(4)
        .then(|point_vec| client.clone().write(point_vec, query.clone()))
        .try_for_each(|_x| futures::future::ready(Ok(())));

    let result = worker.await.unwrap();
    println!("{:?}", result);
}
