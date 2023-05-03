pub mod config;
mod plex;

extern crate dotenv;

use dotenv::dotenv;
use sqlx::SqlitePool;
use std::path::PathBuf;
use std::{env, fs::remove_dir_all};

use crate::config::{read_config, read_param};
use crate::plex::queries;

fn process_removes(folders: Vec<PathBuf>) -> anyhow::Result<()> {
    for folder in folders {
        if folder.exists() {
            println!("removing {}", &folder.to_string_lossy().to_string(),);
            remove_dir_all(folder)?;
        } else {
            println!("'{}' no longer exists, skipping.", folder.to_string_lossy());
        }
    }

    Ok(())
}

fn process_move(folders: Vec<PathBuf>, destination_path: &str) -> anyhow::Result<()> {
    for folder in folders {
        if folder.exists() {
            println!(
                "mv {} {}",
                &folder.to_string_lossy().to_string(),
                &destination_path
            );
            let mut handle = match std::process::Command::new("mv")
                .args([&folder.to_string_lossy().to_string(), destination_path])
                .spawn()
            {
                Ok(h) => h,
                Err(e) => {
                    eprintln!("error in spawned child-process: {}", e);
                    continue;
                }
            };

            if let Err(e) = handle.wait() {
                eprintln!("error in spawned child-process: {}", e);
                continue;
            }
        } else {
            println!("'{}' no longer exists, skipping.", folder.to_string_lossy());
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    for rule in read_config()?.rules {
        println!("processing {} for tag {}", rule.action, rule.tag);
        let folders = queries::folders_by_tag(&pool, &rule.tag).await?;
        println!("found {} matching folders", folders.len());
        // TODO: handled errors
        match rule.action.as_str() {
            "move" => {
                let destination_path = read_param(&rule, "destination_path")?;
                process_move(folders, &destination_path).unwrap();
            }
            "remove" => {
                process_removes(folders).unwrap();
            }
            _ => unreachable!(),
        };
    }

    Ok(())
}
