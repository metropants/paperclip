use dotenv::dotenv;

mod http;

pub const PAPERCLIP: &str = "🍃 Paperclip";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    http::serve().await?;
    Ok(())
}
