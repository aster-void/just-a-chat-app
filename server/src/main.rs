use rocket::*;

mod router;
use router::*;
mod cors;
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

    rocket::build().attach(cors::Cors()).manage(db).mount(
        "/",
        routes![
            root,
            chat::dm_to,
            ws::list_workspaces,
            ws::join_workspace,
            ws::members,
            ws::joined_workspaces,
            ws::get_workspace,
            ws::create_workspace,
            auth::login,
            auth::create_user,
            user::all_user,
            user::user_by_name,
            cors::handle,
        ],
    )
}

#[get("/")]
fn root() -> &'static str {
    "Hello from Rocket! 🚀"
}
