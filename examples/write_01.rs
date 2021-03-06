use influxdb2::write::WriteQuery;
use influxdb2::Client;
use influxdb_line_protocol::{Field, Point};
use std::env;

use futures::{FutureExt, TryFutureExt};
use futures_01::future::Future as Future01;
use tokio_01;

fn main() {
    let host = env::var("INFLUX_HOST").unwrap_or("http://localhost:9999".into());
    let org = env::var("INFLUX_ORG").expect("Expected organization name!");
    let token = env::var("INFLUX_TOKEN").expect("Expected token");
    let bucket = env::var("INFLUX_BUCKET").expect("bucket");

    let client = Client::new(host, org.clone(), token).unwrap();

    let field = Field::new("filedKey", "fieldValue").unwrap();
    let point = Point::builder("test")
        .unwrap()
        .add_field(field)
        .build()
        .unwrap();

    let query = WriteQuery::with_org(&bucket, &org);
    let result = client.write(point, query).boxed().compat();
    let result = result.map_err(drop).map(drop);
    tokio_01::run(result);
}
