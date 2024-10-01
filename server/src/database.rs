use anyhow::Result;
use sqlx::postgres;

pub type Database = postgres::PgPool;

pub trait Borrow {
    fn borrowed(&self) -> &Self {
        &self
    }
}
impl Borrow for Database {}

pub async fn init_db(url: &str, max_conn: u32) -> Result<Database> {
    let pool = postgres::PgPoolOptions::new()
        .max_connections(max_conn)
        .connect(url)
        .await?;
    Ok(pool)
}
