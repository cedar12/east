
use std::time::Duration;
use tokio::time::{self, Instant};

pub struct Throttler {
  max_speed: u64,
  bytes_sent: u64,
  last_update: Instant,
}

impl Throttler {
  pub fn new(max_speed: u64) -> Self {
      Self {
          max_speed,
          bytes_sent: 0,
          last_update: Instant::now(),
      }
  }

  pub async fn throttle(&mut self, bytes: u64, elapsed: Duration) {
      let time_since_last_update = Instant::now().duration_since(self.last_update);
      if time_since_last_update > Duration::from_secs(1) {
          self.bytes_sent = 0;
          self.last_update = Instant::now();
      } else {
        //   let bits_sent = self.bytes_sent * 8;
        //   let bits_per_second = bits_sent / time_since_last_update.as_secs();
        //   if bits_per_second >= self.max_speed {
        //       let target_bits_per_second = self.max_speed - (bits_per_second - self.max_speed);
        //       let target_bytes_per_second = target_bits_per_second / 8;
        //       let delay = Duration::from_secs_f64((bytes as f64) / (target_bytes_per_second as f64));
        //       time::sleep(delay - elapsed).await;
        //       self.bytes_sent = 0;
        //       self.last_update = Instant::now();
        //   }
        let bytes_per_second = self.bytes_sent / time_since_last_update.as_secs();
        if bytes_per_second >= self.max_speed {
            let target_bytes_per_second = self.max_speed - (bytes_per_second - self.max_speed);
            let delay = Duration::from_secs_f64((bytes as f64) / (target_bytes_per_second as f64));
            time::sleep(delay - elapsed).await;
            self.bytes_sent = 0;
            self.last_update = Instant::now();
        }
      }
      self.bytes_sent += bytes;
  }
}
