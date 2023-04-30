extern crate dotenv;

use dotenv::dotenv;
use sqlx::SqlitePool;
use std::{
    collections::HashSet,
    env,
    path::{Path, PathBuf}, fs::remove_dir_all,
};

async fn query_paths_for_tag(
    pool: &SqlitePool,
    target_tag: &str,
) -> anyhow::Result<HashSet<PathBuf>> {
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
    .fetch_all(pool)
    .await?;

    let unique_paths: HashSet<PathBuf> = records
        .into_iter()
        .filter_map(|record| {
            let file_path = record.file?;
            Some(Path::new(&file_path).parent()?.to_path_buf())
        })
        .collect();

    Ok(unique_paths)
}

async fn process_additions(pool: &SqlitePool) -> anyhow::Result<()> {
    println!("processing additions");

    let target_tag = "+";
    let destination_path = env::var("DESTINATION_PATH")?;

    for path in query_paths_for_tag(&pool, &target_tag).await? {
        if path.exists() {
            println!(
                "mv {} {}",
                &path.to_string_lossy().to_string(),
                &destination_path
            );
            let mut handle = match std::process::Command::new("mv")
                .args([&path.to_string_lossy().to_string(), &destination_path])
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
                path.to_string_lossy()
            );
        }
    }

    Ok(())
}

async fn process_removes(pool: &SqlitePool) -> anyhow::Result<()> {
    println!("processing deletes");

    let target_tag = "-";

    for path in query_paths_for_tag(&pool, &target_tag).await? {
        if path.exists() {
            println!(
                "removing {}",
                &path.to_string_lossy().to_string(),
            );
            remove_dir_all(path)?;
        } else {
            println!(
                "'{}' no longer exists, skipping.",
                path.to_string_lossy()
            );
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")?;

    let pool = SqlitePool::connect(&db_url).await?;

    process_additions(&pool).await?;
    process_removes(&pool).await?;

    Ok(())
}
