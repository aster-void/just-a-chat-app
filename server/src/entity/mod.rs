use rocket::{serde::Deserialize, serde::Serialize, FromForm};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Workspace {
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize, FromForm)]
#[serde(crate = "rocket::serde")]
pub struct InitWorkspace {
    pub name: String,
}

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
