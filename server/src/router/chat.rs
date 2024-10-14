use http::Status;
use rocket::serde::json::Json;
use rocket::*;
use server::entity::*;
use sqlx::query_as;

use crate::database::Database;

#[post("/<workspace_id>/chat/create", data = "<chan>")]
pub async fn create_channel(
    workspace_id: i32,
    chan: Json<InitRoom>,
    db: &State<Database>,
) -> Result<Json<Channel>, Status> {
    match query_as!(
        Channel,
        "INSERT INTO channels (name, workspace_id, is_dm) VALUES ($1, $2, false) RETURNING *",
        chan.name,
        workspace_id,
    )
    .fetch_one(db.pool())
    .await
    {
        Ok(t) => Ok(Json(t)),
        Err(err) => {
            eprintln!("router/chat.rs::create_channel - {err}");
            Err(Status::InternalServerError)
        }
    }
}
