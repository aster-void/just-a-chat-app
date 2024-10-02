use rocket::form::Form;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use rocket::{get, post};

use server::*;

use crate::database::Database;

#[get("/workspace")]
pub async fn list_workspaces(db: &State<Database>) -> Result<Json<Vec<Workspace>>, Status> {
    let res = sqlx::query_as!(Workspace, "SELECT * FROM workspaces")
        .fetch_all(db.pool())
        .await;

    match res {
        Ok(val) => Ok(Json(val)),
        Err(err) => {
            println!("{}", err);
            Err(Status::InternalServerError)
        }
    }
}

#[post("/workspace", data = "<workspace>")]
pub async fn create_workspace(
    workspace: Form<InitWorkspace>,
    db: &State<Database>,
) -> Result<Json<Workspace>, Status> {
    let res = sqlx::query_as!(
        Workspace,
        "INSERT INTO workspaces (name) VALUES ($1) RETURNING *",
        workspace.name
    )
    .fetch_one(db.pool())
    .await;

    match res {
        Err(err) => {
            println!("{}", err);
            Err(Status::InternalServerError)
        }
        Ok(val) => Ok(Json(val)),
    }
}
