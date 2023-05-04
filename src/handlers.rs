use std::{fs::remove_dir_all, path::Path};

use anyhow::anyhow;
use log::info;

use crate::config::{read_param, ConfigRule};

fn remove_handler(folder: &Path) -> anyhow::Result<()> {
    info!("removing {}", &folder.to_string_lossy());
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
        "remove" => remove_handler(folder),
        _ => Err(anyhow!("'{}' is not a valid action", rule.action)),
    }
}
