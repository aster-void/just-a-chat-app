use rocket::{serde::Deserialize, serde::Serialize, FromForm};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Workspace {
    pub id: i32,
    pub name: String,
    pub public: bool,
}

#[derive(Deserialize, FromForm)]
#[serde(crate = "rocket::serde")]
pub struct InitWorkspace {
    pub name: String,
    pub public: bool,
}
