use super::UserId;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
};

#[derive(Clone, Copy)]
pub struct AuthenticatedUser(UserId);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();
    async fn from_request(req: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        let Some(token) = req.headers().get_one("Auth-Token") else {
            eprintln!("no Auth-Token header found");
            return Outcome::Error((Status::Unauthorized, ()));
        };

        // TODO: actually implement the logic instead of parsing header as i32
        match token.parse::<i32>() {
            Ok(tok) if tok > 0 => Outcome::Success(AuthenticatedUser(tok)),
            _ => {
                eprintln!("failed to parse Auth-Token");
                Outcome::Error((Status::BadRequest, ()))
            }
        }
    }
}

impl AuthenticatedUser {
    pub fn id(&self) -> UserId {
        self.0
    }
}
