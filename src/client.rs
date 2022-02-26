use http::Method;
use serde::de::DeserializeOwned;
use std::sync::{Arc, RwLock};
use url::Url;

use reqwest::{Client, RequestBuilder};

use crate::{
    builder::GritlabBuilder, config::Config, repo::Repository, user::User, Error,
    Result,
};

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

        Ok(self
            .cli
            .request(method, url)
            .header(auth_header.0, auth_header.1))
    }
}

// API
impl Gritlab {
    // ===============================================
    // User related apis
    // ===============================================

    /// Get the user who owns the auth_token
    pub async fn current_user(&self) -> Result<User> {
        let resp = self.request(Method::GET, "user")?.send().await?;
        resp_json(resp, "get user failed").await
    }

    // ===============================================
    // Repository related apis
    // ===============================================
    //
    /// List all the repos which the user has permission to
    pub async fn list_repos(&self) -> Result<Vec<Repository>> {
        let resp = self.request(Method::GET, "projects")?.send().await?;
        resp_json(resp, "list repos failed").await
    }

    /// Get the specified repo
    pub async fn get_repo(&self, owner: &str, repo: &str) -> Result<Repository> {
        let resp = self
            .request(Method::GET, &format!("projects/{}%2F{}", owner, repo))?
            .send()
            .await?;
        resp_json(resp, "get repo failed").await
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

pub async fn debug_resp(resp: reqwest::Response, start: usize) {
    println!("{:#?}", &resp.text().await.unwrap()[start..start + 50]);
}
