use std::{fs::remove_dir_all, path::Path};

use anyhow::anyhow;
use log::info;

use crate::config::{read_param, ConfigRule};

fn remove_handler(folder: &Path) -> anyhow::Result<()> {
    info!("deleting {}", &folder.to_string_lossy());
    remove_dir_all(folder)?;
    Ok(())
}

fn move_handler(folder: &Path, destination_path: &str) -> anyhow::Result<()> {
    info!("mv {} {}", &folder.to_string_lossy(), &destination_path);

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
    use std::{
        collections::HashMap,
        fs::{create_dir, remove_dir_all},
        path::PathBuf,
        str::FromStr,
    };

    use crate::{config::ConfigRule, handlers::handle_folder};

    #[test]
    fn test_remove_handler() {
        let path = PathBuf::from_str("./test_remove").unwrap();
        create_dir(&path).unwrap();

        handle_folder(
            &ConfigRule {
                action: "delete".into(),
                tag: "-".into(),
                params: None,
            },
            &path,
        )
        .unwrap();

        assert_eq!(path.exists(), false);
    }

    struct Setup;

    impl Drop for Setup {
        fn drop(&mut self) {
            remove_dir_all("test_move").ok();
            remove_dir_all("dest_dir").ok();
        }
    }

    #[test]
    fn test_move_handler() {
        let _setup = Setup;

        create_dir("test_move").unwrap();
        create_dir("dest_dir").unwrap();

        let mut params = HashMap::new();
        params.insert("destination_path".into(), "./dest_dir".into());

        let rule = ConfigRule {
            action: "move".into(),
            tag: "+".into(),
            params: Some(params),
        };

        let path = PathBuf::from_str("./test_move").unwrap();
        handle_folder(&rule, &path).unwrap();
        assert_eq!(path.exists(), false);
        assert_eq!(
            PathBuf::from_str("./dest_dir/test_move").unwrap().exists(),
            true
        );
    }
}
