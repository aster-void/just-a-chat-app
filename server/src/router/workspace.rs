use rocket::form::Form;
use rocket::post;
use rocket::serde::json::Json;

use server::*;

#[post("/workspace", data = "<workspace>")]
pub fn create_workspace(
    method: rocket::http::Method,
    workspace: Form<InitWorkspace>,
) -> Json<Workspace> {
    println!("{method}");
    let res = Workspace {
        id: "hello".into(),
        name: workspace.name.clone(),
    };
    Json(res)
}
