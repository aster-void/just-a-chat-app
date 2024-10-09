use rocket::serde::json::Json;
use rocket::*;
use server::{InitUser, User};

use crate::database::Database;

#[post("/user", data = "<body>")]
pub async fn create_user(body: Json<InitUser>, db: &State<Database>) -> Json<User> {
    match sqlx::query_as!(
        User,
        "INSERT INTO users (name) VALUES ($1) RETURNING *",
        body.name
    )
    .fetch_one(db.pool())
    .await
    {
        Ok(val) => Json(val),
        Err(err) => panic!("{}", err),
    }
}