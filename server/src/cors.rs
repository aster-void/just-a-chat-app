use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Header,
    Request, Response,
};

pub struct Cors();

use std::sync::LazyLock;
static ALLOW_ORIGINS: LazyLock<Vec<String>> = std::sync::LazyLock::new(|| {
    use std::env::var;
    var("CORS_ALLOW_ORIGIN")
        .expect("Couldn't find env CORS_ALLOW_ORIGIN")
        .split(",")
        .map(|s| s.to_owned())
        .collect::<Vec<_>>()
});

#[rocket::options("/<_..>")]
pub fn handle() {}

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        let Some(origin_header) = req.headers().get("ORIGIN").next() else {
            return;
        };
        let Some(res_origin) = ALLOW_ORIGINS.iter().find(|origin| *origin == origin_header) else {
            return; // origin not allowed, just let the browser do the blocking for us
        };
        res.set_header(Header::new("Access-Control-Allow-Origin", res_origin));
        res.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET,HEAD,POST,PUT,PATCH,DELETE,OPTIONS",
        ));
        res.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        res.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
