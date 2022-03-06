use std::collections::HashMap;

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

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub web_url: String,
    pub avatar_url: Option<String>,
    pub git_ssh_url: String,
    pub git_http_url: String,
    pub namespace: String,
    pub visibility_level: i32,
    pub path_with_namespace: String,
    pub default_branch: String,
    pub ci_config_path: Option<String>,
    pub homepage: Option<String>,
    pub url: String,
    pub ssh_url: String,
    pub http_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Author {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Commit {
    pub id: String,
    pub message: String,
    pub title: String,
    pub timestamp: DateTime<Utc>,
    pub url: String,
    pub author: Author,
    pub added: Vec<String>,
    pub modified: Vec<String>,
    pub removed: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Repository {
    pub name: String,
    pub url: String,
    pub description: String,
    pub homepage: String,
    pub git_http_url: String,
    pub git_ssh_url: String,
    pub visibility_level: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PushPayload {
    pub object_kind: String,
    pub event_name: String,
    pub before: String,
    pub after: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub checkout_sha: String,
    pub message: Option<String>,
    pub user_id: i64,
    pub user_name: String,
    pub user_email: String,
    pub user_avatar: Option<String>,
    pub project_id: i64,
    pub project: Project,
    pub commits: Vec<Commit>,
    pub total_commits_count: i64,
    // TODO
    pub push_options: HashMap<String, String>,
    pub repository: Repository,
}
