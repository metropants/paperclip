use clap::Parser;
use config::Config;
use dotenv::dotenv;
use sqlx::PgPool;

mod config;
mod http;

pub const PAPERCLIP: &str = "🍃 Paperclip";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let config = Config::parse();

    let database_url = &config.database_url;
    let pool = PgPool::connect(database_url).await?;

    sqlx::migrate!().run(&pool).await?;

    http::serve(pool).await?;
    Ok(())
}
