use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::State;
use rocket::{get, post};

use server::entity::*;

use crate::database::Database;

#[get("/workspace")]
pub async fn list_workspaces(db: &State<Database>) -> Result<Json<Vec<Workspace>>, Status> {
    let res = sqlx::query_as!(Workspace, "SELECT * FROM workspaces")
        .fetch_all(db.pool())
        .await;

    match res {
        Ok(val) => Ok(Json(val)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/workspace/<id>")]
pub async fn get_workspace(id: i32, db: &State<Database>) -> Result<Json<Workspace>, Status> {
    let res = sqlx::query_as!(Workspace, "SELECT * FROM workspaces WHERE id = $1", id)
        .fetch_one(db.pool())
        .await;

    match res {
        Ok(val) => Ok(Json(val)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/workspace/joined")]
pub async fn joined_workspaces(db: &State<Database>) -> Result<Json<Vec<Workspace>>, Status> {
    sqlx::query_as!(Workspace, "SELECT * FROM workspaces")
        .fetch_all(db.pool())
        .await
        .map(|val| Json(val))
        .map_err(|_| Status::InternalServerError)
}

#[post("/workspace", data = "<workspace>")]
pub async fn create_workspace(
    workspace: Json<InitWorkspace>,
    db: &State<Database>,
) -> Result<Custom<Json<Workspace>>, Status> {
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
        Ok(val) => Ok(Custom(Status::Created, Json(val))),
    }
}
