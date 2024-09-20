use rocket::*;

mod router;
use router::*;
mod database;

#[launch]
fn rocket() -> _ {
    let db = database::init_db();
    rocket::build()
        .manage(db)
        .mount("/", routes![root, chat::dm_to, workspace::create_workspace])
}

#[get("/")]
fn root() -> &'static str {
    "Hello from Rocket! 🚀"
}
