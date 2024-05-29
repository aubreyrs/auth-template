use crate::util::config::Config;

pub fn is_master_key_valid(config: &Config, key: &str) -> bool {
    config.master_key == key
}
