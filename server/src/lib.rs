use rocket::{serde::Serialize, FromForm};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Workspace {
    pub id: i32,
    pub name: String,
}

#[derive(FromForm)]
pub struct InitWorkspace {
    pub name: String,
}
