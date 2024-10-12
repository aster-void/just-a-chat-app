use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::*;
use server::entity::User;

use crate::database::Database;

#[get("/users")]
pub async fn all_user(db: &State<Database>) -> Json<Vec<User>> {
    match sqlx::query_as!(User, "SELECT id, name FROM users")
        .fetch_all(db.pool())
        .await
    {
        Ok(val) => Json(val),
        Err(err) => panic!("{}", err),
    }
}

#[get("/users?<name>")]
pub async fn user_by_name(name: &str, db: &State<Database>) -> Status {
    let result = sqlx::query!(
        r#"SELECT EXISTS (SELECT 1 FROM users WHERE name = $1) AS "exists!""#,
        name
    )
    .fetch_one(db.pool())
    .await;
    match result {
        Ok(record) if record.exists => Status::Ok,
        Ok(_) => Status::NotFound,
        Err(_) => Status::InternalServerError,
    }
}
