pub mod json;
pub mod model;
pub mod mongo;

use crate::util::config::{Config, StorageType};
use model::User;

pub async fn save_user(config: &Config, user: &User) {
    match config.storage {
        StorageType::Mongo => mongo::save_user(config, user).await,
        StorageType::Json => json::save_user(user),
    }
}

pub async fn find_user(config: &Config, username: &str) -> Option<User> {
    match config.storage {
        StorageType::Mongo => mongo::find_user(config, username).await,
        StorageType::Json => json::find_user(username),
    }
}

pub async fn update_user(config: &Config, user: &User) {
    match config.storage {
        StorageType::Mongo => mongo::update_user(config, user).await,
        StorageType::Json => json::update_user(user),
    }
}
