pub mod queries {
    use sqlx::SqlitePool;
    use std::{
        collections::HashSet,
        path::{Path, PathBuf},
    };

    pub async fn folders_by_tag(
        pool: &SqlitePool,
        target_tag: &str,
    ) -> anyhow::Result<Vec<PathBuf>> {
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

        Ok(unique_paths.into_iter().collect())
    }
}
