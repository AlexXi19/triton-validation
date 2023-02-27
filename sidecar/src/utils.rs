use std::time::Duration;

use anyhow::Result;
use tracing::debug;

// Use this at your own risk, there is no timeout
// If you have an unintended error that you didn't expect, using this can be confusing
pub async fn repeat_until_success<F, Fut, T>(operation: F, delay: Duration) -> Result<T>
where
    F: Fn() -> Fut + Send + Sync,
    Fut: std::future::Future<Output = Result<T>> + Send,
    T: Send + 'static,
{
    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(error) => {
                debug!("Operation failed, {}", error);
                tokio::time::sleep(delay).await;
            }
        }
    }
}
