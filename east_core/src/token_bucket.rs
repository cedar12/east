use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::{sleep, Duration};

pub struct TokenBucket {
    rate: f64,
    capacity: usize,
    semaphore: Arc<Semaphore>,
}

impl TokenBucket {
    pub fn new(rate: f64, capacity: usize) -> Self {
        let semaphore = Arc::new(Semaphore::new(capacity));
        Self { rate, capacity, semaphore }
    }

    pub async fn take(&self, n: usize) {
        if n > self.capacity {
            sleep(Duration::from_secs_f64(n as f64 / self.rate)).await;
            return;
        }
        let tokens = self.semaphore.clone();
        for _ in 0..n {
            tokens.acquire().await.unwrap().forget();
        }
        sleep(Duration::from_secs_f64(n as f64 / self.rate)).await;
    }
}