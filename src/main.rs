pub mod config;
mod handlers;
mod plex;

extern crate dotenv;

use dotenv::dotenv;
use log::{error, info};
use sqlx::SqlitePool;
use std::env;
use std::path::PathBuf;

use crate::config::read_config;
use crate::handlers::handle_folder;
use crate::plex::queries;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::init();

    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    for rule in read_config()?.rules {
        info!("processing {} for tag {}", rule.action, rule.tag);

        let (existing_folders, missing_folders): (Vec<PathBuf>, Vec<PathBuf>) =
            queries::folders_by_tag(&pool, &rule.tag)
                .await?
                .into_iter()
                .partition(|f| f.exists());

        missing_folders.iter().for_each(|f| {
            info!(
                "'{}' is missing from file system, skipping.",
                f.to_string_lossy()
            );
        });

        existing_folders
            .iter()
            .filter(|f| f.exists())
            .for_each(|folder| {
                if let Err(e) = handle_folder(&rule, folder) {
                    error!("{}", e);
                }
            });
    }

    Ok(())
}
