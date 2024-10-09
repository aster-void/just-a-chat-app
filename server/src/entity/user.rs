use rocket::{serde::Deserialize, serde::Serialize};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct InitUser {
    pub name: String,
}
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    pub name: String,
}
