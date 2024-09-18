use rocket::*;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![root])
}

#[get("/")]
fn root() -> &'static str {
    "Hello from Rocket! 🚀"
}
