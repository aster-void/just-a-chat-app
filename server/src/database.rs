use anyhow::Result;
use sqlx::postgres;

pub struct Database(postgres::PgPool);

impl Database {
    pub fn pool(&self) -> &postgres::PgPool {
        &self.0
    }
}

pub async fn init_db(url: &str, max_conn: u32) -> Result<Database> {
    let pool = postgres::PgPoolOptions::new()
        .max_connections(max_conn)
        .connect(url)
        .await?;
    Ok(Database(pool))
}
