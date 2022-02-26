use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Hook {
    pub id: i32,
    pub url: String,
    pub project_id: i32,
    pub push_events: bool,
    pub push_events_branch_filter: Option<String>,
    pub issues_events: bool,
    pub confidential_issues_events: Option<bool>,
    pub merge_requests_events: bool,
    pub repository_update_events: bool,
    pub tag_push_events: bool,
    pub note_events: bool,
    pub confidential_note_events: Option<bool>,
    pub job_events: bool,
    pub pipeline_events: bool,
    pub wiki_page_events: bool,
    pub deployment_events: bool,
    pub releases_events: bool,
    pub enable_ssl_verification: bool,
    pub created_at: DateTime<Utc>,
}
