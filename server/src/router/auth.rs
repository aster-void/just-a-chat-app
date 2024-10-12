use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::*;
use serde::{Deserialize, Serialize};
use server::entity::{InitUser, User};

use crate::database::Database;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
// TODO: should AuthInfo be password based?
pub struct AuthInfo {
    pub username: String,
    pub password: String,
}

const BCRYPT_COST: u32 = bcrypt::DEFAULT_COST;

#[post("/signup", data = "<body>")]
pub async fn create_user(
    body: Json<InitUser>,
    db: &State<Database>,
) -> Result<Custom<Json<User>>, Status> {
    let bcrypt_pass = match bcrypt::hash(&body.password, BCRYPT_COST) {
        Ok(hash) => hash,
        Err(err) => {
            // WARNING: since we log password, the client should NOT pass user input password directly as password
            // (a simple SHA256 with some global public pepper would be enough)
            eprintln!(
                "router/auth.rs::create_user - Failed to hash password {} with error {err}",
                body.password
            );
            return Err(Status::InternalServerError);
        }
    };

    match sqlx::query_as!(
        User,
        "INSERT INTO users (name, bcrypt_pass) VALUES ($1, $2) RETURNING id, name",
        body.name,
        bcrypt_pass,
    )
    .fetch_one(db.pool())
    .await
    {
        Ok(val) => Ok(Custom(Status::Created, Json(val))),
        Err(err) => {
            eprintln!("router/auth.rs::create_user - Failed to create user: {err}");
            Err(Status::InternalServerError)
        }
    }
}

struct PullBcryptHash {
    bcrypt_pass: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginResponse {
    token: String,
    user: User,
}

#[post("/login", data = "<body>")]
pub async fn login(
    body: Json<AuthInfo>,
    db: &State<Database>,
) -> Result<Json<LoginResponse>, Status> {
    let hash = match sqlx::query_as!(
        PullBcryptHash,
        "SELECT bcrypt_pass FROM users WHERE name = $1",
        body.username,
    )
    .fetch_optional(db.pool())
    .await
    {
        Ok(Some(hash)) => hash,
        Ok(None) => return Err(Status::NotFound),
        Err(err) => {
            eprintln!("router/auth.rs::login - Failed to fetch data from db: {err}",);
            return Err(Status::InternalServerError);
        }
    };

    match bcrypt::verify(&body.password, &hash.bcrypt_pass) {
        Ok(false) => Err(Status::NotFound),
        Ok(true) => {
            // TODO!("successfully authenticated, publish auth token")
            let user = sqlx::query_as!(
                User,
                "SELECT id, name FROM users WHERE name = $1",
                body.username
            )
            .fetch_one(db.pool())
            .await
            .map_err(|_| Status::InternalServerError)?;
            Ok(Json(LoginResponse {
                token: user.id.to_string(),
                user,
            }))
        }
        Err(err) => {
            eprintln!("router/auth.rs::login() - Failed to verify bcrypt password: {err}",);
            Err(Status::InternalServerError)
        }
    }
}
