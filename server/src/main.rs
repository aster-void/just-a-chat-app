use rocket::*;

mod router;
use router::*;
mod database;
use dotenvy::dotenv;
use workspace as ws;

#[launch]
async fn rocket() -> _ {
    dotenv().expect(".env not found");
    let db_url = std::env::var("DATABASE_URL").expect("Couldn't find database url in env");
    let db = database::init_db(&db_url, 3)
        .await
        .expect("Failed to initialize database");

    rocket::build().manage(db).mount(
        "/",
        routes![root, chat::dm_to, ws::create_workspace, ws::list_workspaces],
    )
}

#[get("/")]
fn root() -> &'static str {
    "Hello from Rocket! 🚀"
}
