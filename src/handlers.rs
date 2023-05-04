use std::{fs::remove_dir_all, path::Path};

use anyhow::anyhow;

use crate::config::{read_param, ConfigRule};

fn remove_handler(folder: &Path) -> anyhow::Result<()> {
    remove_dir_all(folder)?;
    Ok(())
}

fn move_handler(folder: &Path, destination_path: &str) -> anyhow::Result<()> {
    std::process::Command::new("mv")
        .args([&folder.to_string_lossy(), destination_path])
        .spawn()?
        .wait()?;
    Ok(())
}

pub fn handle_folder(rule: &ConfigRule, folder: &Path) -> anyhow::Result<()> {
    match rule.action.as_str() {
        "move" => move_handler(folder, &read_param(rule, "destination_path")?),
        "delete" => remove_handler(folder),
        _ => Err(anyhow!("'{}' is not a valid action", rule.action)),
    }
}

#[cfg(test)]
mod tests {
    use crate::{config::ConfigRule, handlers::handle_folder};
    use std::collections::HashMap;
    use tempfile::tempdir;

    #[test]
    fn test_remove_handler() {
        let dir = tempdir().unwrap();

        assert_eq!(dir.path().exists(), true);
        handle_folder(
            &ConfigRule {
                action: "delete".into(),
                tag: "-".into(),
                params: None,
            },
            &dir.path(),
        )
        .unwrap();
        assert_eq!(dir.path().exists(), false);
    }

    #[test]
    fn test_move_handler() {
        let dir = tempdir().unwrap();
        let destination_dir = tempdir().unwrap();

        let mut params = HashMap::new();
        params.insert(
            "destination_path".into(),
            destination_dir.path().to_str().unwrap().into(),
        );

        let rule = ConfigRule {
            action: "move".into(),
            tag: "+".into(),
            params: Some(params),
        };

        assert_eq!(dir.path().exists(), true);
        handle_folder(&rule, &dir.path()).unwrap();
        assert_eq!(dir.path().exists(), false);
        assert_eq!(destination_dir.path().exists(), true);
    }
}
