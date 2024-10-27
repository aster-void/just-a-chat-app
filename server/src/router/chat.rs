use chrono;
use http::Status;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket::*;
use server::entity::*;
use sqlx::{query, query_as};

use crate::database::Database;

#[post("/<workspace_id>/chat", data = "<chan>")]
pub async fn create_channel(
    workspace_id: i32,
    chan: Json<InitRoom>,
    req: AuthenticatedUser,
    db: &State<Database>,
) -> Result<Json<Channel>, Status> {
    match sqlx::query!(
        "SELECT EXISTS (SELECT 1 FROM belongs WHERE belongs.user_id = $1 AND belongs.workspace_id = $2) as \"exists!\"",
        req.id(),
        workspace_id,
    )
    .fetch_one(db.pool())
    .await {
    Err(err) => {
        eprintln!("{err}");
        return Err(Status::InternalServerError);
    }
    Ok(record) => {
        if !record.exists {
            return Err(Status::Forbidden);
        }
    },
    }

    match query_as!(
        Channel,
        "INSERT INTO channels (name, workspace_id, is_dm) VALUES ($1, $2, false) RETURNING *",
        chan.name,
        workspace_id,
    )
    .fetch_one(db.pool())
    .await
    {
        Err(err) => {
            eprintln!("router/chat.rs::create_channel - {err}");
            Err(Status::InternalServerError)
        }
        Ok(t) => match query!(
            "INSERT INTO member_of (user_id, channel_id) VALUES ($1, $2)",
            req.id(),
            t.id
        )
        .execute(db.pool())
        .await
        {
            Ok(_) => Ok(Json(t)),
            Err(err) => {
                eprintln!("{err}");
                Err(Status::InternalServerError)
            }
        },
    }
}

#[get("/<workspace_id>/chat")]
pub async fn list_channels(
    workspace_id: i32,
    db: &State<Database>,
) -> Result<Json<Vec<Channel>>, Status> {
    query_as!(
        Channel,
        "SELECT * FROM channels WHERE channels.workspace_id = $1",
        workspace_id
    )
    .fetch_all(db.pool())
    .await
    .map(Json)
    .map_err(|err| {
        eprintln!("router/chat.rs::list_channels - {err}");
        Status::InternalServerError
    })
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
        return Err(Status::Forbidden);
    }
    query_as!(
        Message,
        "INSERT INTO messages (content, posted_at, posted_workspace, posted_chan, posted_by) VALUES ($1, $2, $3, $4, $5) RETURNING *",
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
