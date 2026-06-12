use std::time::Duration;
use tokio::time::sleep;

/// Retry a function multiple times with a delay between attempts
pub async fn retry_with_delay<F, T, E>(mut f: F, max_attempts: u32, delay: Duration) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
{
    for attempt in 0..max_attempts {
        match f() {
            Ok(result) => return Ok(result),
            Err(e) if attempt == max_attempts - 1 => return Err(e),
            Err(_) => sleep(delay).await,
        }
    }
    unreachable!()
}

/// Retry an async function multiple times with a delay between attempts
pub async fn retry_async<F, Fut, T, E>(mut f: F, max_attempts: u32, delay: Duration) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
{
    for attempt in 0..max_attempts {
        match f().await {
            Ok(result) => return Ok(result),
            Err(e) if attempt == max_attempts - 1 => return Err(e),
            Err(_) => sleep(delay).await,
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_retry_async() {
        let count = Arc::new(AtomicU32::new(0));
        let count_clone = count.clone();

        let result = retry_async(
            || {
                let c = count_clone.clone();
                async move {
                    let current = c.fetch_add(1, Ordering::SeqCst) + 1;
                    if current < 3 {
                        Err("not yet")
                    } else {
                        Ok("success")
                    }
                }
            },
            5,
            Duration::from_millis(10),
        )
        .await;

        assert_eq!(result, Ok("success"));
        assert_eq!(count.load(Ordering::SeqCst), 3);
    }
}
