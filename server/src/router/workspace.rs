use rocket::form::Form;
use rocket::post;
use rocket::serde::json::Json;
use rocket::State;

use server::*;

use crate::database::Database;

#[post("/workspace", data = "<workspace>")]
pub fn create_workspace(workspace: Form<InitWorkspace>, db: &State<Database>) -> Json<Workspace> {
    let res = Workspace {
        id: format!("id-randomly-generated-uuidv7-or-something-{}", db.obtain()),
        name: workspace.name.clone(),
    };
    Json(res)
}
