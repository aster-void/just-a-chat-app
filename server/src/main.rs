use rocket::*;

mod router;
use router::*;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![root, chat::dm_to, workspace::create_workspace])
}

#[get("/")]
fn root() -> &'static str {
    "Hello from Rocket! 🚀"
}
