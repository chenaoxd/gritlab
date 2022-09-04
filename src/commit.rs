use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ListCommitsOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_stats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_parent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailers: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Stat {
    pub additions: i64,
    pub deletions: i64,
    pub total: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Commit {
    pub id: String,
    pub short_id: String,
    pub title: String,
    pub author_name: String,
    pub author_email: String,
    pub authored_date: DateTime<Utc>,
    pub committer_name: String,
    pub committer_email: String,
    pub committed_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub message: String,
    pub parent_ids: Vec<String>,
    pub web_url: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<Stat>,
}
