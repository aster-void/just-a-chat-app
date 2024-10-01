use rocket::form::Form;
use rocket::post;
use rocket::serde::json::Json;
use rocket::State;

use server::*;

use crate::database::{Borrow, Database};

#[post("/workspace", data = "<workspace>")]
pub async fn create_workspace(
    workspace: Form<InitWorkspace>,
    db: &State<Database>,
) -> Json<Workspace> {
    let pool = db.borrowed();
    let res = sqlx::query_as!(
        Workspace,
        "INSERT INTO workspaces (name) VALUES ($1) RETURNING *",
        workspace.name
    )
    .fetch_one(pool)
    .await;

    match res {
        Err(err) => panic!("{}", err), // todo
        Ok(ws) => Json(ws.into()),
    }
}
