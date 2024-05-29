use actix_web::{web, HttpResponse};
use serde::Deserialize;
use log::{info, warn};

use crate::data::{self};
use crate::util::hash::verify_password;
use crate::util::config::Config;

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

pub async fn login(config: web::Data<Config>, item: web::Json<LoginRequest>) -> HttpResponse {
    if let Some(user) = data::find_user(&config, &item.username).await {
        if user.suspended {
            warn!("Suspended user attempted to log in: {}", item.username);
            return HttpResponse::Forbidden().json("User account is suspended");
        }
        if verify_password(&user.password_hash, &item.password) {
            info!("User logged in: {}", item.username);
            return HttpResponse::Ok().json("Login successful");
        }
    }

    warn!("Invalid login attempt for username: {}", item.username);
    HttpResponse::Unauthorized().json("Invalid username or password")
}
