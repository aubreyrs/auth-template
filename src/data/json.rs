use super::model::User;
use std::fs::File;
use std::io::{Read, Write};

const FILE_PATH: &str = "users.json";

pub fn save_user(user: &User) {
    let mut users = load_users();
    users.push(user.clone());
    let json = serde_json::to_string(&users).unwrap();
    let mut file = File::create(FILE_PATH).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

pub fn load_users() -> Vec<User> {
    if let Ok(mut file) = File::open(FILE_PATH) {
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        serde_json::from_str(&contents).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    }
}

pub fn find_user(username: &str) -> Option<User> {
    load_users().into_iter().find(|u| u.username == username)
}

pub fn update_user(updated_user: &User) {
    let mut users = load_users();
    for user in &mut users {
        if user.username == updated_user.username {
            *user = updated_user.clone();
        }
    }
    let json = serde_json::to_string(&users).unwrap();
    let mut file = File::create(FILE_PATH).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}
