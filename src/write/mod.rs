use super::id::Id;
use super::precision::Precision;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WriteQuery {
    bucket: String,
    // at least one is required!
    org: Option<String>,
    #[serde(rename = "orgID")]
    org_id: Option<Id>,

    precision: Precision,
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

    pub fn precision_mut(&mut self) -> &mut Precision {
        &mut self.precision
    }

    pub fn precision(mut self, precision: Precision) -> Self {
        self.precision = precision;
        self
    }
}
