use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Debug, Deserialize, Serialize)]
pub struct Hook {
    pub id: i64,
    pub url: String,
    pub project_id: i64,
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

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateHookOption {
    pub url: String,
    pub confidential_issues_events: Option<bool>,
    pub confidential_note_events: Option<bool>,
    pub deployment_events: Option<bool>,
    pub enable_ssl_verification: Option<bool>,
    pub issues_events: Option<bool>,
    pub job_events: Option<bool>,
    pub merge_requests_events: Option<bool>,
    pub note_events: Option<bool>,
    pub pipeline_events: Option<bool>,
    pub push_events_branch_filter: Option<String>,
    pub push_events: Option<bool>,
    pub releases_events: Option<bool>,
    pub tag_push_events: Option<bool>,
    pub token: Option<String>,
    pub wiki_page_events: Option<bool>,
}

impl CreateHookOption {
    pub fn new(url: &str, token: Option<String>) -> Self {
        Self {
            url: url.to_string(),
            confidential_issues_events: None,
            confidential_note_events: None,
            deployment_events: None,
            enable_ssl_verification: Some(true),
            issues_events: None,
            job_events: None,
            merge_requests_events: None,
            note_events: None,
            pipeline_events: None,
            push_events_branch_filter: None,
            push_events: Some(true),
            releases_events: None,
            tag_push_events: None,
            token,
            wiki_page_events: None,
        }
    }
}
