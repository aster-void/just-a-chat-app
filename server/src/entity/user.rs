use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct InitUser {
    pub name: String,
    pub password: String,
}
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    pub name: String,
}

pub type UserId = i32;
