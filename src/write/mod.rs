use super::{error::Error, id::Id, Client};
use influxdb_line_protocol::{Batch, Precision};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WriteQuery {
    bucket: String,
    // at least one is required!
    org: Option<String>,
    #[serde(rename = "orgID")]
    org_id: Option<Id>,

    // precision will be added from Timestamp
    precision: Option<Precision>,
}

impl WriteQuery {
    pub fn with_org(bucket: impl Into<String>, org: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            org: Some(org.into()),
            org_id: None,
            precision: Default::default(),
        }
    }

    pub fn with_org_id(bucket: impl Into<String>, org_id: impl Into<Id>) -> Self {
        Self {
            bucket: bucket.into(),
            org: None,
            org_id: Some(org_id.into()),
            precision: Default::default(),
        }
    }

    /// This should be set from `Batch::precision` function.
    pub(crate) fn set_precision(&mut self, precision: Option<Precision>) {
        self.precision = precision;
    }
}

pub struct WriteFactory<T>
where
    T: Clone + Into<Batch>,
{
    client: Client,
    data: T,
    query: WriteQuery,
}

impl<T> WriteFactory<T>
where
    T: Clone + Into<Batch> + Send,
{
    pub fn new(client: Client, data: T, query: WriteQuery) -> Self {
        Self {
            client,
            data,
            query,
        }
    }

    pub async fn write_req(&self) -> Result<(), Error> {
        self.client
            .clone()
            .write(self.data.clone(), self.query.clone())
            .await
    }
}
