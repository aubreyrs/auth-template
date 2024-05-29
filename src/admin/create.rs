use actix_web::{web, HttpResponse};
use serde::Deserialize;
use log::{info, warn};

use crate::data::{self, model::User};
use crate::util::hash::hash_password;
use crate::util::masterkey::is_master_key_valid;
use crate::util::config::Config;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    master_key: String,
    username: String,
    password: String,
    email: String,
}

pub async fn create_user(config: web::Data<Config>, item: web::Json<CreateUserRequest>) -> HttpResponse {
    if !is_master_key_valid(&config, &item.master_key) {
        warn!("Invalid master key attempt for creating user: {}", item.username);
        return HttpResponse::Forbidden().json("Invalid master key");
    }

    let password_hash = hash_password(&item.password);
    let user = User {
        username: item.username.clone(),
        password_hash,
        email: item.email.clone(),
        suspended: false,
    };

    data::save_user(&config, &user).await;
    info!("User created: {}", item.username);

    HttpResponse::Ok().json("User created")
}
