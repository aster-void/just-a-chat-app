use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::State;
use rocket::{get, post};

use server::entity::*;

use sqlx::{query, query_as};

use crate::database::Database;

#[get("/workspace?<joined>")]
pub async fn list_workspaces(
    joined: Option<bool>,
    db: &State<Database>,
    login_state: Option<AuthenticatedUser>,
) -> Result<Json<Vec<Workspace>>, Status> {
    let pool = db.pool();
    match (login_state, joined) {
        (None, Some(false)) => query_as!(Workspace, "SELECT * FROM workspaces").fetch_all(pool).await,
        (None, Some(true)) => Ok(Vec::new()), // a user who's not logged in does not belong to any workspace
        (_, None) => query_as!(Workspace, "SELECT * FROM workspaces").fetch_all(pool).await,
        (Some(user), Some(false))=> 
            query_as!(Workspace,
                "SELECT * FROM workspaces WHERE NOT EXISTS
                (SELECT * FROM belongs WHERE belongs.workspace_id = workspaces.id AND belongs.user_id = $1)",
                user.id()
            ).fetch_all(pool).await,
        (Some(user), Some(true)) => 
            query_as!(Workspace,
                "SELECT * FROM workspaces WHERE EXISTS
                (SELECT * FROM belongs WHERE belongs.workspace_id = workspaces.id AND belongs.user_id = $1)",
                user.id()
            ).fetch_all(pool).await,
    }
    .map(Json).map_err(|err| {
        eprintln!("router/workspace.rs::list_workspaces - {err}");
        Status::InternalServerError
    })
}

#[post("/workspace", data = "<workspace>")]
pub async fn create_workspace(
    workspace: Json<InitWorkspace>,
    db: &State<Database>,
    user: AuthenticatedUser,
) -> Result<Custom<Json<Workspace>>, Status> {
    let create = query_as!(
        Workspace,
        "INSERT INTO workspaces (name) VALUES ($1) RETURNING *",
        workspace.name,
    )
    .fetch_one(db.pool())
    .await;

    match create {
        Err(err) => {
            eprintln!("router/workspace.rs::create_workspace - {err}");
            return Err(Status::InternalServerError);
        }
        Ok(ws) => {
            match query!(
                "INSERT INTO belongs (workspace_id, user_id) VALUES ($1, $2)",
                ws.id,
                user.id()
            )
            .execute(db.pool())
            .await
            {
                Ok(_) => Ok(Custom(Status::Created, Json(ws))),
                Err(err) => {
                    eprintln!("{}", err);
                    Err(Status::InternalServerError)
                }
            }
        }
    }
}

#[get("/workspace/<workspace_id>")]
pub async fn get_workspace(
    workspace_id: i32,
    db: &State<Database>,
    auth: AuthenticatedUser,
) -> Result<Json<Workspace>, Status> {
    let res = query_as!(
        Workspace,
        "SELECT * FROM workspaces WHERE id = $1 AND EXISTS
        (SELECT * FROM belongs WHERE belongs.user_id = $2 AND belongs.workspace_id = workspaces.id)",
        workspace_id,
        auth.id()
    )
    .fetch_optional(db.pool())
    .await;

    match res {
        Ok(Some(val)) => Ok(Json(val)),
        Ok(None) => Err(Status::NotFound),
        Err(err) => {
            eprintln!("router/workspace.rs::get_workspace - {err}");
            Err(Status::InternalServerError)
        },
    }
}

#[post("/workspace/join/<workspace_id>")]
pub async fn join_workspace(
    workspace_id: i32,
    db: &State<Database>,
    user: AuthenticatedUser,
) -> Status {
    let result = query!(
        "INSERT INTO belongs (user_id, workspace_id) VALUES ($1, $2)",
        user.id(),
        workspace_id
    )
    .execute(db.pool())
    .await;

    match result {
        Ok(_) => Status::Created,
        Err(err) => {
            eprintln!("router/workspace.rs::join_workspace - {err}");
            Status::InternalServerError
        },
    }
}

#[get("/workspace/<workspace_id>/members")]
pub async fn members(workspace_id: i32, db: &State<Database>) -> Result<Json<Vec<User>>, Status> {
    let result = query_as!(
        User,
        "SELECT id, name FROM users WHERE EXISTS
            (SELECT * FROM belongs WHERE belongs.user_id = users.id AND belongs.workspace_id = $1)",
        workspace_id
    )
    .fetch_all(db.pool())
    .await;

    match result {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(Status::InternalServerError),
    }
}
