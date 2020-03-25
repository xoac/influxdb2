use crate::error::{ApiError, Error, InvalidUri};
use http::{method::Method, uri::Uri};
use influxdb_line_protocol::Batch;
use log::trace;
use reqwest as rw;
use reqwest::Client as ReqwestClient;
use serde::de::DeserializeOwned;
use std::sync::Arc;

/// Influx DB v2 Client
#[derive(Debug, Clone)]
pub struct Client {
    inner: Arc<InnerClient>,
}

#[derive(Debug)]
struct InnerClient {
    hyper: ReqwestClient,
    token: String,
    org: String,
    // http://example.org:9999/api/v2/
    base_url: String,
}

async fn json_or_error<T>(result: rw::Response) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    if result.status().is_success() {
        result.json::<T>().await.map_err(Error::from)
    } else {
        let api_err = result.json::<ApiError>().await?;
        Err(Error::from(api_err))
    }
}

impl Client {
    /// Create new Influx DB Client
    pub fn new<T>(url: T, org: T, token: T) -> Result<Self, InvalidUri>
    where
        T: AsRef<str>,
    {
        // check url is correct
        let _uri: Uri = url.as_ref().parse()?;

        let inner = Arc::new(InnerClient {
            hyper: Default::default(),
            token: token.as_ref().into(),
            org: org.as_ref().into(),
            base_url: format!("{}/api/v2/", url.as_ref()),
        });

        Ok(Self { inner })
    }

    fn req_builder(&self, method: Method, url: impl AsRef<str>) -> rw::RequestBuilder {
        self.inner
            .hyper
            .request(method, url.as_ref())
            .header("Authorization", format!("Token {}", self.inner.token))
    }
}

use crate::bucket::Buckets;
/// Implement Buckets
///
/// All method from [bucket doc api](https://v2.docs.influxdata.com/v2.0/api/#tag/Buckets)
impl Client {
    ///List all buckets.
    ///
    /// [source doc](https://v2.docs.influxdata.com/v2.0/api/#operation/GetBuckets)
    pub async fn bucket_list_all(&self) -> Result<Buckets, Error> {
        let uri = format!("{}{}", self.inner.base_url, "buckets");
        println!("Sedning req {}", uri);
        let result = self.req_builder(Method::GET, uri).send().await?;
        json_or_error::<Buckets>(result).await
    }
}

use crate::write::WriteQuery;
/// Implement write
///
/// All method form [write doc api](https://v2.docs.influxdata.com/v2.0/api/#tag/Write)
impl Client {
    //TODO in v0.1.0 change `self` and `query` to reference. Stop support compat write.
    pub async fn write<B>(self, batch: B, query: WriteQuery) -> Result<(), Error>
    where
        B: Into<Batch> + Send,
    {
        let batch = batch.into();
        let str_lines = batch.to_line_protocol();
        trace!("Write body in protocol-line:\n {}", str_lines);
        let uri = format!("{}{}", self.inner.base_url, "write");

        let result = self
            .req_builder(Method::POST, uri)
            .query(&query)
            .body(str_lines)
            .send()
            .await?;
        if result.status().is_success() {
            Ok(())
        } else {
            let api_err = result.json::<ApiError>().await?;
            Err(Error::from(api_err))
        }
    }
}
