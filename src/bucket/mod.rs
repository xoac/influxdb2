use super::{id::Id, timestamp::Timestamp};
use serde::{Deserialize, Serialize};

mod retention_rule;
use retention_rule::RetentionRule;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bucket {
    id: Id,
    r#type: String,
    created_at: Timestamp,
    updated_at: Timestamp,
    //labels: Vec<Label>,

    // this fields are shared with crate request
    name: String,
    description: Option<String>,
    #[serde(rename = "orgID")]
    org_id: Option<Id>,
    rp: Option<String>,
    retention_rules: Vec<RetentionRule>,
}

#[derive(Debug, Serialize)]
pub struct CreateBucket {
    name: String,
    description: String,
    #[serde(rename = "orgID")]
    org_id: Id,
    rp: String,
    retention_rules: Vec<RetentionRule>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Buckets {
    // links
    buckets: Vec<Bucket>,
}
