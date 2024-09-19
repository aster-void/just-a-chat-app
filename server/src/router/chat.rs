use rocket::*;

#[post("/<workspace>/dm/<to>")]
pub fn dm_to(workspace: &str, to: &str) -> String {
    println!("in {workspace} to {to}");
    "Hello from sub path".into()
}
