use anyhow::anyhow;
use serde::Deserialize;
use std::{collections::HashMap, fs};

#[derive(Deserialize)]
pub struct ConfigRule {
    pub tag: String,
    pub action: String,
    pub params: Option<HashMap<String, String>>,
}

#[derive(Deserialize)]
pub struct Config {
    pub rules: Vec<ConfigRule>,
}

pub fn read_config() -> anyhow::Result<Config> {
    let cfg_yaml = fs::read_to_string("./config.yaml")?;
    serde_yaml::from_str(&cfg_yaml).map_err(|e| {
        println!("{}", e);
        anyhow!("failed to read config")
    })
}

pub fn read_param(rule: &ConfigRule, param: &str) -> anyhow::Result<String> {
    let params = rule
        .params
        .clone()
        .ok_or(anyhow!("cannot access `params` of rule {}.", rule.tag))?;

    params
        .get(param)
        .ok_or(anyhow!("cannot access param {param} of rule {}", rule.tag))
        .cloned()
}
