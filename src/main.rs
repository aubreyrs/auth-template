use actix_web::{web, App, HttpServer};
use env_logger::Env;

mod data;
mod util;
mod user;
mod admin;

use util::config::Config;
use user::{signup, login};
use admin::{create, suspend, passreset};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let config = Config::from_file();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .route("/signup", web::post().to(signup::signup))
            .route("/login", web::post().to(login::login))
            .route("/admin/create", web::post().to(create::create_user))
            .route("/admin/suspend", web::post().to(suspend::suspend_user))
            .route("/admin/passreset", web::post().to(passreset::reset_password))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
