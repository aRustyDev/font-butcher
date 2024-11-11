use std::env;
use std::collections::BTreeMap;

// TODO: Handle missing env vars

pub fn get_env_vars() -> Result<BTreeMap<String, Option<String>>, Box<dyn std::error::Error>> {
    let mut env_vars = BTreeMap::new();
    env_vars.insert("INPUT_FILE".to_string(), check_env_vars("INPUT_FILE")?);
    env_vars.insert("OUTPUT_FILE".to_string(), check_env_vars("OUTPUT_FILE")?);
    env_vars.insert("BASE64_PASSWORD".to_string(), check_env_vars("BASE64_PASSWORD")?);

    Ok(env_vars)
}

fn check_env_vars(var: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    match env::var(var) {
        Ok(this) => Ok(Some(this)),
        Err(_) => Ok(None),
    }
}