use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Namespace {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub kind: String,
    pub full_path: String,
    pub parent_id: Option<i64>,
    pub avatar_url: Option<String>,
    pub web_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Links {
    #[serde(rename = "self")]
    pub self_: String,
    pub issues: String,
    pub merge_requests: String,
    pub repo_branches: String,
    pub labels: String,
    pub events: String,
    pub members: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContainerExpiryPolicy {
    pub cadence: String,
    pub enabled: bool,
    pub keep_n: i64,
    pub older_than: String,
    pub name_regex: String,
    pub name_regex_keep: Option<String>,
    pub next_run_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Owner {
    pub id: i64,
    pub name: String,
    pub username: String,
    pub state: String,
    pub avatar_url: String,
    pub web_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Group {
    pub group_id: i64,
    pub group_name: String,
    pub group_full_path: String,
    pub group_access_level: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Access {
    access_level: i64,
    notification_level: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Permissions {
    pub project_accesss: Option<Access>,
    pub group_access: Option<Access>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Repository {
    pub id: i64,
    pub description: String,
    pub name: String,
    pub name_with_namespace: String,
    pub path: String,
    pub path_with_namespace: String,
    pub created_at: DateTime<Utc>,
    pub default_branch: String,
    pub tag_list: Vec<String>,
    pub ssh_url_to_repo: String,
    pub http_url_to_repo: String,
    pub web_url: String,
    pub readme_url: Option<String>,
    pub avatar_url: Option<String>,
    pub forks_count: i64,
    pub star_count: i64,
    pub last_activity_at: DateTime<Utc>,
    pub namespace: Namespace,
    pub _links: Links,
    pub packages_enabled: bool,
    pub empty_repo: bool,
    pub archived: bool,
    pub visibility: String,
    pub owner: Option<Owner>,
    pub resolve_outdated_diff_discussions: bool,
    pub container_registry_enabled: bool,
    pub container_expiration_policy: ContainerExpiryPolicy,
    pub issues_enabled: bool,
    pub merge_requests_enabled: bool,
    pub wiki_enabled: bool,
    pub jobs_enabled: bool,
    pub snippets_enabled: bool,
    pub service_desk_enabled: bool,
    pub service_desk_address: Option<String>,
    pub can_create_merge_request_in: bool,
    pub issues_access_level: String,
    pub repository_access_level: String,
    pub merge_requests_access_level: String,
    pub forking_access_level: String,
    pub wiki_access_level: String,
    pub builds_access_level: String,
    pub snippets_access_level: String,
    pub pages_access_level: String,
    pub operations_access_level: String,
    pub analytics_access_level: String,
    pub emails_disabled: Option<bool>,
    pub shared_runners_enabled: bool,
    pub lfs_enabled: bool,
    pub creator_id: i64,
    pub import_status: String,
    pub open_issues_count: i64,
    pub ci_default_git_depth: i64,
    pub ci_forward_deployment_enabled: bool,
    pub public_jobs: bool,
    pub build_timeout: i64,
    pub auto_cancel_pending_pipelines: String,
    pub build_coverage_regex: Option<String>,
    pub ci_config_path: Option<String>,
    pub shared_with_groups: Vec<Group>,
    pub only_allow_merge_if_pipeline_succeeds: bool,
    pub allow_merge_on_skipped_pipeline: Option<bool>,
    pub restrict_user_defined_variables: bool,
    pub request_access_enabled: bool,
    pub only_allow_merge_if_all_discussions_are_resolved: bool,
    pub remove_source_branch_after_merge: bool,
    pub printing_merge_request_link_enabled: bool,
    pub merge_method: String,
    pub suggestion_commit_message: Option<String>,
    pub auto_devops_enabled: bool,
    pub auto_devops_deploy_strategy: String,
    pub autoclose_referenced_issues: bool,
    pub repository_storage: String,
    // TODO
    // pub requirements_enabled: Option<String>,
    pub security_and_compliance_enabled: bool,
    pub compliance_frameworks: Vec<String>,
    pub permissions: Permissions,
}
