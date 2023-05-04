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
    let cfg_yaml = fs::read_to_string("./config.yml")?;
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

#[cfg(test)]
mod tests {
    use super::ConfigRule;
    use crate::config::read_param;
    use std::collections::HashMap;

    #[test]
    fn test_read_param_present() {
        let mut params = HashMap::new();
        params.insert("foo".into(), "bar".into());
        let rule = ConfigRule {
            tag: "++".into(),
            action: "move".into(),
            params: Some(params),
        };

        assert_eq!(read_param(&rule, "foo").unwrap(), "bar");
        assert_eq!(read_param(&rule, "baz").is_err(), true);
    }

    #[test]
    fn test_read_param_empty() {
        let rule = ConfigRule {
            tag: "++".into(),
            action: "move".into(),
            params: None,
        };

        assert_eq!(read_param(&rule, "baz").is_err(), true);
    }
}
