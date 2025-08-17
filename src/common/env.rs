/// Get an env variable or from .env
pub fn get_setting(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| dotenvy::var(key).expect(&*format!("Could not get the '{key}' env var")))
}