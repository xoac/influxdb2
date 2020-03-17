use influxdb2::write::WriteQuery;
use influxdb2::Client;
use influxdb_line_protocol::{error::Error, Field, Point};
use std::convert::TryFrom;

struct Meas {
    u: u64,
    i: i64,
    fi: f64,
    //tags
    location: String,
}

impl Into<Point> for Meas {
    fn into(self) -> Point {
        let fields = vec![
            Field::new("u", self.u).unwrap(),
            Field::new("i", self.i).unwrap(),
            Field::try_from(("fi", self.fi)).unwrap(),
        ];

        Point::builder("meas")
            .try_add_tag::<_, Error>(("location", self.location))
            .unwrap()
            .add_fields(fields)
            .build()
            .unwrap()
    }
}

#[tokio::main]
async fn main() {
    let client = Client::new(
        "http://localhost:9999",
        "org",
        "X1eNV3hVXYoha3yI3U8P9f2IaoN8oKLJc5vnvfi_i2xCGLTvszSl3y5eqw9kcFFFNiVXDaLfNuvyzeJdqxBT1w==",
    )
    .unwrap();

    let meas = Meas {
        u: 44,
        i: 45i64,
        fi: 47f64,
        location: "T".to_owned(),
    };

    let query = WriteQuery::with_org("bucket", "org");
    let result = client.write(meas, query).await;
    println!("{:?}", result);
}
