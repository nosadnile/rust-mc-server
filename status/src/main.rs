use anyhow::Result;
use status::status::fetch_status;

#[tokio::main]
pub async fn main() -> Result<()> {
    let status = fetch_status("localhost", Some(25565)).await?;

    println!("{:?}", status);

    Ok(())
}
