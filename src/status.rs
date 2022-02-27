use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Author {
    pub id: i32,
    pub name: String,
    pub username: String,
    pub state: String,
    pub avatar_url: Option<String>,
    pub web_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Status {
    pub id: i32,
    pub sha: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub status: String,
    pub name: String,
    pub target_url: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub started_at: DateTime<Utc>,
    pub finished_at: DateTime<Utc>,
    pub allow_failure: bool,
    pub coverage: Option<f32>,
    pub author: Author,
}
