use std::env;

use anyhow::Context;
use gritlab::{
    client::Gritlab, hook::CreateHookOption, status::CreateStatusOption, Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    let (owner, repo) = ("chenao", "test-jarvis");
    let commit_id = "ff0e6ddd616ffabfc02d6943b2aed496fca2c63c";

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

    cli.delete_hook(owner, repo, hook.id).await?;
    println!("hook deleted");

    let status = cli
        .create_status(
            owner,
            repo,
            commit_id,
            &CreateStatusOption {
                state: "failed".to_string(),
                ref_: None,
                context: Some("jarvis".to_string()),
                target_url: Some("https://jarvis.chenaoxd.com/repo/1/jobs".to_string()),
                description: Some("some description".to_string()),
                coverage: None,
                pipeline_id: None,
            },
        )
        .await?;
    println!("created status: {:#?}", status);

    let statuses = cli.list_statuses(owner, repo, commit_id).await?;
    println!("statuses: {:#?}", statuses);

    Ok(())
}
