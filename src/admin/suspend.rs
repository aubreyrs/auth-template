use actix_web::{web, HttpResponse};
use serde::Deserialize;
use log::{info, warn};

use crate::data;
use crate::util::masterkey::is_master_key_valid;
use crate::util::config::Config;

#[derive(Deserialize)]
pub struct SuspendUserRequest {
    master_key: String,
    username: String,
}

pub async fn suspend_user(config: web::Data<Config>, item: web::Json<SuspendUserRequest>) -> HttpResponse {
    if !is_master_key_valid(&config, &item.master_key) {
        warn!("Invalid master key attempt for suspending user: {}", item.username);
        return HttpResponse::Forbidden().json("Invalid master key");
    }

    if let Some(mut user) = data::find_user(&config, &item.username).await {
        user.suspended = true;
        data::update_user(&config, &user).await;
        info!("User suspended: {}", item.username);
        return HttpResponse::Ok().json("User suspended");
    }

    warn!("Suspension attempt for non-existent user: {}", item.username);
    HttpResponse::NotFound().json("User not found")
}
