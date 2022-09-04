pub mod auth;
pub mod builder;
pub mod client;
pub mod commit;
pub mod config;
pub mod error;
pub mod hook;
pub mod repo;
pub mod status;
pub mod user;

use chrono::{DateTime, Utc};
pub use error::{Error, Result};
use serde::Serializer;

pub fn gitlab_datetime_fmt<S>(
    datetime_opt: &Option<DateTime<Utc>>,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match datetime_opt {
        None => serializer.serialize_str("nil"),
        Some(datetime) => {
            serializer.serialize_str(&format!("{}", datetime.format("YYYY-MM-DDTHH:MM:SSZ")))
        }
    }
}
