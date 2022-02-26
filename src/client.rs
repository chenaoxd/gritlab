use http::Method;
use serde::de::DeserializeOwned;
use std::sync::{Arc, RwLock};
use url::Url;

use reqwest::{Client, RequestBuilder};

use crate::{builder::GritlabBuilder, config::Config, user::User, Error, Result};

#[derive(Debug, Clone)]
pub struct Gritlab {
    conf: Arc<RwLock<Config>>,
    cli: Client,
}

impl Gritlab {
    pub fn builder(host: impl Into<String>) -> GritlabBuilder {
        GritlabBuilder::new(host)
    }

    pub fn new(conf: Config, cli: Client) -> Self {
        Self {
            conf: Arc::new(RwLock::new(conf)),
            cli,
        }
    }

    pub fn r_conf(&self) -> Result<Config> {
        Ok(self.conf.read()?.clone())
    }

    pub fn abs_url(&self, rel_url: &str) -> Result<Url> {
        Ok(self.r_conf()?.base_url.join(rel_url)?)
    }

    pub fn api_url(&self, rel_url: &str) -> Result<Url> {
        Ok(self.abs_url("api/v4/")?.join(rel_url)?)
    }

    pub fn headers(&self) -> Result<(String, String)> {
        // TODO: auto refresh
        self.r_conf()?.token.headers()
    }

    pub fn request(&self, method: Method, rel_url: &str) -> Result<RequestBuilder> {
        let url = self.api_url(rel_url)?;
        let auth_header = self.headers()?;

        println!("{:#?}", auth_header);

        Ok(self
            .cli
            .request(method, url)
            .header("Content-Type", "application/json")
            .header(auth_header.0, auth_header.1))
    }
}

// API
impl Gritlab {
    // ===============================================
    // User related apis
    // ===============================================
    pub async fn current_user(&self) -> Result<User> {
        let resp = self.request(Method::GET, "user")?.send().await?;

        resp_json(resp, "get user failed").await
    }

    // ===============================================
    // Repository related apis
    // ===============================================
    pub async fn list_repos(&self) {
        todo!()
    }
}

pub async fn resp_json<T>(resp: reqwest::Response, err_mes: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    if !resp.status().is_success() {
        Err(Error::GitlabError(format!(
            "{}: [{}] {}",
            err_mes,
            resp.status(),
            resp.text().await?
        )))
    } else {
        Ok(resp.json::<T>().await?)
    }
}

pub async fn check_success(resp: reqwest::Response, err_mes: &str) -> Result<()> {
    if !resp.status().is_success() {
        Err(Error::GitlabError(format!(
            "{}: [{}] {}",
            err_mes,
            resp.status(),
            resp.text().await?
        )))
    } else {
        Ok(())
    }
}
