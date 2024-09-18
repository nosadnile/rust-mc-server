use anyhow::Result;
use nsn_work::init;

#[tokio::main]
pub async fn main() -> Result<()> {
    init().await
}
