extern crate dotenv;

use dotenv::dotenv;
use sqlx::SqlitePool;
use std::{
    collections::HashSet,
    env,
    path::{Path, PathBuf},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")?;
    let destination_path = env::var("DESTINATION_PATH")?;
    let target_tag = env::var("TARGET_TAG")?;

    let pool = SqlitePool::connect(&db_url).await?;

    let records = sqlx::query!(
        "
        SELECT media_parts.file
        FROM media_parts
        JOIN media_items ON media_parts.media_item_id == media_items.id
        JOIN metadata_items ON media_items.metadata_item_id == metadata_items.id
        WHERE metadata_items.parent_id
        IN (
            SELECT metadata_item_id
            FROM taggings
            WHERE tag_id = (
                SELECT id
                FROM tags
                WHERE tag_type = 2
                AND tag = ?
            )
        )
        ",
        target_tag
    )
    .fetch_all(&pool)
    .await?;

    if records.is_empty() {
        println!("did not find any items for tag '{}'.", target_tag);
        return Ok(());
    }

    let unique_paths: HashSet<PathBuf> = records
        .into_iter()
        .filter_map(|record| {
            let file_path = record.file?;
            Some(Path::new(&file_path).parent()?.to_path_buf())
        })
        .collect();

    for file_path in unique_paths {
        if file_path.exists() {
            println!("mv {} {}", &file_path.to_string_lossy().to_string(), &destination_path);
            let mut handle = match std::process::Command::new("mv")
                .args(&[&file_path.to_string_lossy().to_string(), &destination_path])
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
            println!(
                "'{}' no longer exists, skipping.",
                file_path.to_string_lossy()
            );
        }
    }

    Ok(())
}
