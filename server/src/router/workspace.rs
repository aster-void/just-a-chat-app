use rocket::form::Form;
use rocket::http::Status;
use rocket::post;
use rocket::serde::json::Json;
use rocket::State;

use server::*;

use crate::database::{Borrow, Database};

#[post("/workspace", data = "<workspace>")]
pub async fn create_workspace(
    workspace: Form<InitWorkspace>,
    db: &State<Database>,
) -> Result<Json<Workspace>, Status> {
    let pool = db.borrowed();
    let res = sqlx::query_as!(
        Workspace,
        "INSERT INTO workspaces (name) VALUES ($1) RETURNING *",
        workspace.name
    )
    .fetch_one(pool)
    .await;

    match res {
        Err(err) => {
            println!("{}", err);
            Err(Status::InternalServerError)
        }
        Ok(val) => Ok(Json(val)),
    }
}
