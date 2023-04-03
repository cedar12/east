use std::time::Duration;

use tokio::{time::Instant, sync::Semaphore};

pub struct TokenBucket {
    capacity: usize,
    tokens: usize,
    last_refill: Instant,
    refill_interval: Duration,
    refill_amount: usize,
    semaphore: Semaphore,
}

impl TokenBucket {
    pub fn new(capacity: usize, refill_rate: usize) -> Self {
        let refill_interval = Duration::from_secs(1);
        let refill_amount = refill_rate;
        let semaphore = Semaphore::new(capacity);

        Self {
            capacity,
            tokens: capacity,
            last_refill: Instant::now(),
            refill_interval,
            refill_amount,
            semaphore,
        }
    }

    pub async fn take(&mut self, n: usize) {
        let mut tokens = n;

        loop {
            let available_tokens = self.semaphore.available_permits() as usize;
            let refill_amount = self.refill_amount();

            if available_tokens >= tokens {
                self.semaphore.acquire_many(tokens as u32).await.unwrap();
                break;
            } else if available_tokens > 0 {
                self.semaphore.acquire_many(available_tokens as u32).await.unwrap();
                tokens -= available_tokens;
            } else {
                let time_since_last_refill = Instant::now().duration_since(self.last_refill);
                let tokens_to_add = (time_since_last_refill.as_secs_f64() / self.refill_interval.as_secs_f64())
                    as usize * self.refill_amount;
                let new_tokens = (self.tokens + tokens_to_add).min(self.capacity);
                self.tokens = new_tokens;
                self.last_refill += Duration::from_secs_f64(
                    (new_tokens - self.tokens) as f64 / self.refill_amount as f64
                        * self.refill_interval.as_secs_f64(),
                );
            }
        }
    }

    fn refill_amount(&self) -> usize {
        let time_since_last_refill = Instant::now().duration_since(self.last_refill);
        let tokens_to_add = (time_since_last_refill.as_secs_f64() / self.refill_interval.as_secs_f64())
            as usize * self.refill_amount;
        (self.tokens + tokens_to_add).min(self.capacity) - self.tokens
    }
}