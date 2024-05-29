use mongodb::{Client, options::ClientOptions, Collection};
use super::model::User;
use bson::doc;
use crate::util::config::Config;

pub async fn get_user_collection(config: &Config) -> Collection<User> {
    let client_options = ClientOptions::parse(config.mongodb_connection_string.as_ref().unwrap())
        .await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let database = client.database("silly");
    database.collection::<User>("users")
}

pub async fn save_user(config: &Config, user: &User) {
    let collection = get_user_collection(config).await;
    collection.insert_one(user, None).await.unwrap();
}

pub async fn find_user(config: &Config, username: &str) -> Option<User> {
    let collection = get_user_collection(config).await;
    collection.find_one(doc! { "username": username }, None).await.unwrap()
}

pub async fn update_user(config: &Config, user: &User) {
    let collection = get_user_collection(config).await;
    collection.update_one(
        doc! { "username": &user.username },
        doc! { "$set": { "suspended": user.suspended } },
        None
    ).await.unwrap();
}
