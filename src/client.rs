use crate::error::ApiError;
use http::{method::Method, request::Builder as ReqBuilder, uri::Uri};
use influxdb_line_protocol::Batch;
use reqwest as rw;
use reqwest::Client as ReqwestClient;
use serde_json::Value;

struct ConnectError {}

/// Influx DB v2 Client
#[derive(Debug, Clone)]
pub struct Client {
    hyper: ReqwestClient,
    token: String,
    org: String,
    // http://example.org:9999/api/v2/
    base_url: String,
}

impl Client {
    /// Create new Influx DB Client
    pub fn new<T>(url: T, org: T, token: T) -> Result<Self, http::uri::InvalidUri>
    where
        T: AsRef<str>,
    {
        // check url is correct
        let _uri: Uri = url.as_ref().parse()?;

        Ok(Self {
            hyper: Default::default(),
            token: token.as_ref().into(),
            org: org.as_ref().into(),
            base_url: format!("{}/api/v2/", url.as_ref()),
        })
    }

    fn req_builder(&self, method: Method, url: impl AsRef<str>) -> rw::RequestBuilder {
        self.hyper
            .request(method, url.as_ref())
            .header("Authorization", format!("Token {}", self.token))
    }
}

/// Implement Buckets
///
/// All method from [bucket doc api](https://v2.docs.influxdata.com/v2.0/api/#tag/Buckets)
use crate::bucket::Buckets;
impl Client {
    ///List all buckets.
    ///
    /// [source doc](https://v2.docs.influxdata.com/v2.0/api/#operation/GetBuckets)
    pub async fn bucket_list_all(&self) -> Result<Buckets, rw::Error> {
        let uri = format!("{}{}", self.base_url, "buckets");
        println!("Sedning req {}", uri);
        let result = self.req_builder(Method::GET, uri).send().await?;
        if result.status().is_success() {
            result.json::<Buckets>().await
        } else {
            todo!()
        }
    }
}

use crate::write::WriteQuery;

impl Client {
    pub async fn write(
        &self,
        batch: impl Into<Batch>,
        query: &WriteQuery,
    ) -> Result<(), rw::Error> {
        let batch = batch.into();
        let str_lines = batch.to_line_protocol();
        let uri = format!("{}{}", self.base_url, "write");

        let result = self
            .req_builder(Method::POST, uri)
            .query(query)
            .body(str_lines)
            .send()
            .await?;
        if result.status().is_success() {
            Ok(())
        } else {
            todo!();
        }
    }
}
