use chrono;
use http::Status;
use rocket::response::status::Created;
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

pub struct Exists {
    pub ok: bool,
}

#[post("/<workspace_id>/chat/<chan>/send", data = "<init>")]
pub async fn send_message(
    workspace_id: i32,
    chan: i32,
    init: Json<InitMessage>,
    db: &State<Database>,
    user: AuthenticatedUser,
) -> Result<Created<Json<Message>>, Status> {
    let now = chrono::Utc::now().timestamp();

    let Exists { ok } = query_as!(
        Exists,
        r#"SELECT
        EXISTS (SELECT 1 FROM channels WHERE channels.workspace_id = $1 AND channels.id = $2)
        AND EXISTS (SELECT 1 FROM member_of WHERE member_of.channel_id = $2 AND member_of.user_id = $3)
        AS "ok!""#,
        workspace_id,
        chan,
        user.id(),
    )
    .fetch_one(db.pool())
    .await
    .map_err(|_| Status::InternalServerError)?;
    if !ok {
        return Err(Status::Unauthorized);
    }
    query_as!(
        Message,
        "INSERT INTO messages (content, posted_at, posted_chan, posted_workspace, posted_by) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        init.content, now, workspace_id, chan, user.id()
    )
    .fetch_one(db.pool())
    .await
    .map(|msg| Created::new("path").body(Json(msg)))
    .map_err(|err| {
        eprintln!("router/chat.rs::send_message - {err}");
        Status::InternalServerError
    })
}
