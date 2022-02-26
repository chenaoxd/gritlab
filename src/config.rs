use url::Url;

use crate::auth::Auth;

#[derive(Debug, Clone)]
pub struct Config {
    pub base_url: Url,
    pub token: Auth,
}
