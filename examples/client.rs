use std::env;

use anyhow::Context;
use gritlab::{client::Gritlab, hook::CreateHookOption, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let (owner, repo) = ("contract", "scheduler");

    let host = env::var("HOST")
        .with_context(|| format!("get environment variable HOST failed"))?;
    let access_token = env::var("ACCESS_TOKEN")
        .with_context(|| format!("get environment variable ACCESS_TOKEN failed"))?;

    let cli = Gritlab::builder(host).token(access_token).build()?;

    let user = cli.current_user().await?;
    println!("current_user: {:#?}", user);

    let _repos = cli.list_repos().await?;
    // println!("repos: {:#?}", repos);

    let repo_ = cli.get_repo(owner, repo).await?;
    println!("repo: {:#?}", repo_);

    let hook = cli
        .create_hook(
            owner,
            repo,
            &CreateHookOption::new(
                "https://foo.bar/hook",
                Some("demo_token".to_string()),
            ),
        )
        .await?;
    println!("new hook: {:#?}", hook);

    let hooks = cli.list_hooks(owner, repo).await?;
    println!("hooks: {:#?}", hooks);

    Ok(())
}
