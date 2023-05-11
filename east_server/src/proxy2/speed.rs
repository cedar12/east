use tokio::time::Instant;

pub struct Tachometer{
  bytes_read:u64,
  start_time:Instant,
  bytes_per_sec:u64,
}

impl Tachometer{
  pub fn new()->Self{
    Self { bytes_read: 0, start_time: Instant::now(),bytes_per_sec:0}
  }

  pub fn has(&mut self,length:usize)->bool{
      self.bytes_read += length as u64;
      if self.start_time.elapsed().as_secs() >= 1 {
        let elapsed_secs = self.start_time.elapsed().as_secs();
        self.bytes_per_sec = self.bytes_read / elapsed_secs;
        self.bytes_read = 0;
        self.start_time = Instant::now();
        return true;
      }
      false
  }

  pub fn speed(&self)->u64{
    self.bytes_per_sec
  }
}