use chrono::{Date, DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub username: String,
    pub state: String,
    pub avatar_url: String,
    pub web_url: String,
    pub created_at: DateTime<Utc>,
    pub bio: String,
    pub bio_html: String,
    pub location: String,
    pub public_email: String,
    pub skype: String,
    pub linkedin: String,
    pub twitter: String,
    pub website_url: String,
    pub organization: String,
    pub job_title: String,
    pub bot: bool,
    pub work_information: Option<String>,
    pub followers: i32,
    pub following: i32,
    pub last_sign_in_at: DateTime<Utc>,
    pub confirmed_at: DateTime<Utc>,
    pub last_activity_on: NaiveDate,
    pub email: String,
    pub theme_id: i32,
    pub color_scheme_id: i32,
    pub projects_limit: i32,
    pub current_sign_in_at: DateTime<Utc>,
    // TODO
    // pub identities: Vec<()>,
    pub can_create_group: bool,
    pub can_create_project: bool,
    pub two_factor_enabled: bool,
    pub external: bool,
    pub private_profile: bool,
    // TODO
    // pub shared_runners_minutes_limit: Option<i32>,
    // TODO
    // pub extra_shared_runners_minutes_limit: Option<i32>,
    pub is_admin: bool,
    pub note: String,
    pub using_license_seat: bool,
}
