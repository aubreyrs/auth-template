use actix_web::{web, HttpResponse};
use serde::Deserialize;
use log::info;

use crate::data::{self, model::User};
use crate::util::hash::hash_password;
use crate::util::config::Config;

#[derive(Deserialize)]
pub struct SignupRequest {
    username: String,
    password: String,
    email: String,
}

pub async fn signup(config: web::Data<Config>, item: web::Json<SignupRequest>) -> HttpResponse {
    let password_hash = hash_password(&item.password);
    let user = User {
        username: item.username.clone(),
        password_hash,
        email: item.email.clone(),
        suspended: false,
    };

    data::save_user(&config, &user).await;
    info!("User signed up: {}", item.username);

    HttpResponse::Ok().json("User created")
}
