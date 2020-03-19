use futures::stream::{StreamExt, TryStreamExt};
use influxdb2::{
    error::{Error, ErrorCode},
    write::WriteQuery,
    Client, Precision,
};
use influxdb_line_protocol::*;
use std::env;

fn read_4_varibales() -> (String, String, String, String) {
    let host = env::var("INFLUX_HOST").unwrap_or("http://localhost:9999".into());
    let org = env::var("INFLUX_ORG").expect("INFLUX_ORG env variable must be set");
    let token = env::var("INFLUX_TOKEN").expect("INFLUX_TOKEN env variable must be set");
    let bucket = env::var("INFLUX_BUCKET").expect("INFLUX_BUCKET env variable must be set");

    (host, org, token, bucket)
}

#[tokio::test]
async fn write_to_unexist_bucket() {
    let (host, org, token, _bucket) = read_4_varibales();
    let client = Client::new(host, org.clone(), token).unwrap();

    let field = Field::new("filedKey", "fieldValue").unwrap();
    let point = Point::builder("test")
        .unwrap()
        .add_field(field)
        .build()
        .unwrap();

    let query = WriteQuery::with_org("unexist", &org);
    let err = client.write(point, query).await.unwrap_err();
    let api_err = match err {
        Error::Api(api_err) => api_err,
        _ => panic!("Should be api_err"),
    };
    assert_eq!(api_err.code(), ErrorCode::NotFound);
}

#[tokio::test]
async fn single_point_write() {
    let (host, org, token, bucket) = read_4_varibales();
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
    let (host, org, token, bucket) = read_4_varibales();
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
