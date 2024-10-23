use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct InitRoom {
    pub name: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct InitDM {
    pub to: i32, // references User(id)
    pub workspace_id: i32,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Channel {
    pub id: i32,
    pub name: String,
    pub workspace_id: i32,
    pub is_dm: bool,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct InitMessage {
    pub content: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Message {
    pub id: i32,
    pub content: String,
    pub posted_at: i64,        // DateTime<UTC> encoded as UNIX timestamp
    pub posted_chan: i32,      // references Channel(id)
    pub posted_workspace: i32, // references Workspace(id)
    pub posted_by: i32,        // references User(id)
}
