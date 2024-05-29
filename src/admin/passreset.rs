use actix_web::{web, HttpResponse};
use serde::Deserialize;
use log::{info, warn};

use crate::data;
use crate::util::hash::hash_password;
use crate::util::masterkey::is_master_key_valid;
use crate::util::config::Config;

#[derive(Deserialize)]
pub struct ResetPasswordRequest {
    master_key: String,
    username: String,
    new_password: String,
}

pub async fn reset_password(config: web::Data<Config>, item: web::Json<ResetPasswordRequest>) -> HttpResponse {
    if !is_master_key_valid(&config, &item.master_key) {
        warn!("Invalid master key attempt for resetting password for user: {}", item.username);
        return HttpResponse::Forbidden().json("Invalid master key");
    }

    if let Some(mut user) = data::find_user(&config, &item.username).await {
        user.password_hash = hash_password(&item.new_password);
        data::update_user(&config, &user).await;
        info!("Password reset successfully for user: {}", item.username);
        return HttpResponse::Ok().json("Password reset successfully");
    }

    warn!("Password reset attempt for non-existent user: {}", item.username);
    HttpResponse::NotFound().json("User not found")
}
