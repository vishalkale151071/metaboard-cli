use std::time::Duration;

use tokio::time::sleep;

pub async fn wait() -> anyhow::Result<()>{
    delay(5).await;
    Ok(())
}

async fn delay(seconds: u64) {
    sleep(Duration::from_secs(seconds)).await;
}