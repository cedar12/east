use tokio::sync::mpsc::Receiver;
use tokio::time::Instant;

use crate::byte_buf::ByteBuf;

use crate::handler::{Handler };
use crate::throttler::Throttler;
use crate::token_bucket::TokenBucket;
use crate::{context::Context, decoder::Decoder, encoder::Encoder};
use tokio::io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf, self};
use tokio::sync::{mpsc::channel,Mutex};
use std::net::SocketAddr;
use std::sync::Arc;

const READ_SIZE: usize = 1024*10;

pub struct Bootstrap<E, D, H, T,S>
where
    E: Encoder<T> + Send + 'static,
    D: Decoder<T> + Send + 'static,
    H: Handler<T> + Send + 'static,
    T: Send + Sync + 'static,
    S: AsyncWriteExt + AsyncReadExt
{
    encoder: Arc<Mutex<E>>,
    decoder: D,
    handler: Arc<Mutex<H>>,
    ctx: Arc<Context<T>>,
    in_rv: Arc<Mutex<Receiver<T>>>,
    out_rv: Arc<Mutex<Receiver<T>>>,
    r:Arc<Mutex<ReadHalf<S>>>,
    w:Arc<Mutex<WriteHalf<S>>>,
    close:Arc<Mutex<Receiver<()>>>,
    read_size:usize,
    rate_limit:Arc<Mutex<Option<TokenBucket>>>,
}

impl<E, D, H, T,S> Bootstrap<E, D, H, T,S>
where
    E: Encoder<T> + Send + 'static,
    D: Decoder<T> + Send + 'static,
    H: Handler<T> + Send + 'static,
    T: Send + Sync + 'static,
    S: AsyncWriteExt + AsyncReadExt
{
    pub fn build(stream: S,addr:SocketAddr, e: E, d: D, h: H) -> Self {
        let (in_tx, in_rv) = channel(1024);
        let (out_tx, out_rv) = channel(1024);
        let (close_tx, close_rv) = channel(128);
        let (r,w)=io::split(stream);
        
        Bootstrap {
            encoder: Arc::new(Mutex::new(e)),
            decoder: d,
            handler: Arc::new(Mutex::new(h)),
            ctx: Arc::new(Context::new(in_tx, out_tx,addr,close_tx)),
            in_rv: Arc::new(Mutex::new(in_rv)),
            out_rv: Arc::new(Mutex::new(out_rv)),
            r:Arc::new(Mutex::new(r)),
            w:Arc::new(Mutex::new(w)),
            close:Arc::new(Mutex::new(close_rv)),
            read_size:READ_SIZE,
            rate_limit:Arc::new(Mutex::new(None))
        }
    }

    pub async fn set_rate_limit(&self,rate_limit:u64){
        let bucket = TokenBucket::new(rate_limit as f64, self.read_size*self.read_size);
        self.rate_limit.lock().await.insert(bucket);
    }

    pub fn capacity(&mut self,size:usize){
        self.read_size=size;
    }

    async fn handle_run(&mut self)->std::io::Result<()>{
        let handler = Arc::clone(&self.handler);
        let encoder = Arc::clone(&self.encoder);
        let ctx = Arc::clone(&self.ctx);
        let out_rv = Arc::clone(&self.out_rv);
        let in_rv = Arc::clone(&self.in_rv);

        let r=Arc::clone(&self.r);
        let w=Arc::clone(&self.w);
        
        handler.lock().await.active(ctx.as_ref()).await;
        let close=Arc::clone(&self.close);
      
        let mut bf = ByteBuf::new_with_capacity(self.read_size);
        let mut buf = vec![0u8; self.read_size];
        let ctx = &self.ctx;

        let mut out = out_rv.lock().await;
        let mut in_rv = in_rv.lock().await;
        let mut close = close.lock().await;
        let mut r=r.lock().await;
        
        let limiter=self.rate_limit.lock().await;

        loop {
            tokio::select!{
                msg = out.recv() => {
                    if let Some(msg)=msg{
                        let mut h=handler.lock().await;
                        let h=h.read(ctx.as_ref(), msg);
                        h.await;
                    }
                },
                msg=in_rv.recv()=>{
                    if let Some(msg)=msg{
                        let mut byte_buf = ByteBuf::new_with_capacity(self.read_size);
                        encoder
                            .lock()
                            .await
                            .encode(ctx.as_ref(), msg, &mut byte_buf);
                        let mut buf = vec![0u8; byte_buf.readable_bytes()];
                        byte_buf.read_bytes(&mut buf);
                        
                        w.lock().await.write(&buf).await?;
                    }
                },
                _ = close.recv() => {
                    w.lock().await.shutdown().await?;
                    return Ok(())
                },
                n=r.read(&mut buf)=>{
                    
                    match n{
                        Ok(0)=>{
                            return Ok(())
                        },
                        Ok(n)=>{
                            if n == 0 {
                                return Ok(());
                            }
                            if let Some(limiter)=limiter.as_ref(){
                                limiter.take(n).await;
                            }
                            bf.write_bytes(&buf[..n])?;
                            self.decoder.decode(ctx, &mut bf).await;
                        },
                        Err(e)=>{
                            return Err(e)
                        }
                    }
                }
            }
        }
    }

    pub async fn run(&mut self) -> std::io::Result<()> {
        let ctx = Arc::clone(&self.ctx);
        let result=self.handle_run().await;
        let mut h=self.handler.lock().await;
        h.close(ctx.as_ref()).await;
        result
    }


}



use crate::encoder2::EncoderMut;
use crate::decoder2::DecoderMut;
use crate::handler2::HandlerMut;

pub struct BootstrapMut<E, D, H, T,S>
where
    E: EncoderMut<T> + Send + 'static,
    D: DecoderMut<T> + Send + 'static,
    H: HandlerMut<T> + Send + 'static,
    T: Send + Sync + 'static,
    S: AsyncWriteExt + AsyncReadExt
{
    encoder: Arc<Mutex<E>>,
    decoder: D,
    handler: Arc<Mutex<H>>,
    ctx: Arc<Context<T>>,
    in_rv: Arc<Mutex<Receiver<T>>>,
    out_rv: Arc<Mutex<Receiver<T>>>,
    r:Arc<Mutex<ReadHalf<S>>>,
    w:Arc<Mutex<WriteHalf<S>>>,
    close:Arc<Mutex<Receiver<()>>>,
    read_size:usize,
    throttler:Arc<Mutex<Option<Throttler>>>,
}

impl<E, D, H, T,S> BootstrapMut<E, D, H, T,S>
where
    E: EncoderMut<T> + Send + 'static,
    D: DecoderMut<T> + Send + 'static,
    H: HandlerMut<T> + Send + 'static,
    T: Send + Sync + 'static,
    S: AsyncWriteExt + AsyncReadExt
{
    pub fn build(stream: S,addr:SocketAddr, e: E, d: D, h: H) -> Self {
        let (in_tx, in_rv) = channel(1024);
        let (out_tx, out_rv) = channel(1024);
        let (close_tx, close_rv) = channel(128);
        let (r,w)=io::split(stream);
        
        Self {
            encoder: Arc::new(Mutex::new(e)),
            decoder: d,
            handler: Arc::new(Mutex::new(h)),
            ctx: Arc::new(Context::new(in_tx, out_tx,addr,close_tx)),
            in_rv: Arc::new(Mutex::new(in_rv)),
            out_rv: Arc::new(Mutex::new(out_rv)),
            r:Arc::new(Mutex::new(r)),
            w:Arc::new(Mutex::new(w)),
            close:Arc::new(Mutex::new(close_rv)),
            read_size:READ_SIZE,
            throttler:Arc::new(Mutex::new(None))
        }
    }

    pub async fn set_rate_limit(&self,rate_limit:u64){
        self.throttler.lock().await.insert(Throttler::new(rate_limit));
    }

    pub fn capacity(&mut self,size:usize){
        self.read_size=size;
    }

    async fn handle_run(&mut self)->std::io::Result<()>{
        let handler = Arc::clone(&self.handler);
        let encoder = Arc::clone(&self.encoder);
        let ctx = Arc::clone(&self.ctx);
        let out_rv = Arc::clone(&self.out_rv);
        let in_rv = Arc::clone(&self.in_rv);

        let r=Arc::clone(&self.r);
        let w=Arc::clone(&self.w);
        
        handler.lock().await.active(ctx.as_ref()).await;
        let close=Arc::clone(&self.close);
      
        let mut bf = ByteBuf::new_with_capacity(self.read_size);
        let mut buf = vec![0u8; self.read_size];
        let ctx = &self.ctx;

        let mut out = out_rv.lock().await;
        let mut in_rv = in_rv.lock().await;
        let mut close = close.lock().await;
        let mut r=r.lock().await;
        
        let mut limiter=self.throttler.lock().await;

        loop {
            let mut start_time = Instant::now();
            tokio::select!{
                msg = out.recv() => {
                    if let Some(msg)=msg{
                        let mut h=handler.lock().await;
                        let h=h.read(ctx.as_ref(), msg);
                        h.await;
                    }
                },
                msg=in_rv.recv()=>{
                    if let Some(msg)=msg{
                        let mut byte_buf = ByteBuf::new_with_capacity(self.read_size);
                        encoder
                            .lock()
                            .await
                            .encode(ctx.as_ref(), msg, &mut byte_buf).await;
                        let mut buf = vec![0u8; byte_buf.readable_bytes()];
                        byte_buf.read_bytes(&mut buf);
                        
                        w.lock().await.write(&buf).await?;
                    }
                },
                _ = close.recv() => {
                    w.lock().await.shutdown().await?;
                    return Ok(())
                },
                n=r.read(&mut buf)=>{
                    
                    match n{
                        Ok(0)=>{
                            return Ok(())
                        },
                        Ok(n)=>{
                            if n == 0 {
                                return Ok(());
                            }
                            let elapsed = start_time.elapsed();
                            if let Some(limiter)=limiter.as_mut(){
                                limiter.throttle(n as u64,elapsed).await;
                            }
                            bf.write_bytes(&buf[..n])?;
                            self.decoder.decode(ctx, &mut bf).await;
                            start_time += elapsed;
                        },
                        Err(e)=>{
                            return Err(e)
                        }
                    }
                }
            }
        }
    }

    pub async fn run(&mut self) -> std::io::Result<()> {
        let ctx = Arc::clone(&self.ctx);
        let result=self.handle_run().await;
        let mut h=self.handler.lock().await;
        h.close(ctx.as_ref()).await;
        result
    }


}