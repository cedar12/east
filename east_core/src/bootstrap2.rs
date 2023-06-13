///
/// 减少使用mutex,提升性能
/// 
use tokio::sync::mpsc::Receiver;

use crate::byte_buf::ByteBuf;

use crate::handler::{Handler };
use crate::token_bucket::TokenBucket;
use crate::{context::Context, decoder::Decoder, encoder::Encoder};
use tokio::io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf, self};
use tokio::sync::{mpsc::channel};
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
    encoder: E,
    decoder: D,
    handler: H,
    ctx: Context<T>,
    in_rv: Receiver<T>,
    out_rv: Receiver<T>,
    r:ReadHalf<S>,
    w:WriteHalf<S>,
    close:Receiver<()>,
    read_size:usize,
    rate_limit:Option<TokenBucket>,
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
            encoder: e,
            decoder: d,
            handler: h,
            ctx: Context::new(in_tx, out_tx,addr,close_tx),
            in_rv: in_rv,
            out_rv: out_rv,
            r:r,
            w:w,
            close:close_rv,
            read_size:READ_SIZE,
            rate_limit:None
        }
    }

    pub async fn set_rate_limit(&mut self,rate_limit:u64){
        let bucket = TokenBucket::new(rate_limit as f64, self.read_size*self.read_size);
        self.rate_limit.insert(bucket);
    }

    pub fn capacity(&mut self,size:usize){
        self.read_size=size;
    }

    async fn handle_run(&mut self)->std::io::Result<()>{
        let handler = &mut self.handler;
        let encoder = &mut self.encoder;

        let out_rv = &mut self.out_rv;
        let in_rv = &mut self.in_rv;

        let r=&mut self.r;
        let w=&mut self.w;
        

        let close=&mut self.close;
      
        let mut bf = ByteBuf::new_with_capacity(self.read_size);
        let mut buf = vec![0u8; self.read_size];
        let ctx = &self.ctx;

        handler.active(ctx).await;
        
        let limiter=&mut self.rate_limit;

        loop {
            tokio::select!{
                msg = out_rv.recv() => {
                    if let Some(msg)=msg{
                        handler.read(ctx,msg).await;
                    }
                },
                msg=in_rv.recv()=>{
                    if let Some(msg)=msg{
                        let mut byte_buf = ByteBuf::new_with_capacity(self.read_size);
                        encoder.encode(ctx,msg,&mut byte_buf);
                        let mut buf = vec![0u8; byte_buf.readable_bytes()];
                        byte_buf.read_bytes(&mut buf);
                        w.write(&buf).await?;
                    }
                },
                _ = close.recv() => {
                    w.shutdown().await?;
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
        // let ctx = self.ctx.clone();//Arc::clone(&self.ctx);
        let result=self.handle_run().await;
        self.handler.close(&self.ctx).await;
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
    encoder: E,
    decoder: D,
    handler: H,
    ctx: Arc<Context<T>>,
    in_rv: Receiver<T>,
    out_rv: Receiver<T>,
    r:ReadHalf<S>,
    w:WriteHalf<S>,
    close:Receiver<()>,
    read_size:usize,
    rate_limit:Option<TokenBucket>,
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
        
        BootstrapMut {
            encoder: e,
            decoder: d,
            handler: h,
            ctx: Arc::new(Context::new(in_tx, out_tx,addr,close_tx)),
            in_rv: in_rv,
            out_rv: out_rv,
            r:r,
            w:w,
            close:close_rv,
            read_size:READ_SIZE,
            rate_limit:None
        }
    }

    pub async fn set_rate_limit(&mut self,rate_limit:u64){
        let bucket = TokenBucket::new(rate_limit as f64, self.read_size*self.read_size);
        self.rate_limit.insert(bucket);
    }

    pub fn capacity(&mut self,size:usize){
        self.read_size=size;
    }

    async fn handle_run(&mut self)->std::io::Result<()>{
        let handler = &mut self.handler;
        let encoder = &mut self.encoder;
        
        let out_rv = &mut self.out_rv;
        let in_rv = &mut self.in_rv;

        let r=&mut self.r;
        let w=&mut self.w;
        

        let close=&mut self.close;
      
        let mut bf = ByteBuf::new_with_capacity(self.read_size);
        let mut buf = vec![0u8; self.read_size];
        let ctx = &self.ctx;

        handler.active(ctx.as_ref()).await;
        
        let limiter=&mut self.rate_limit;

        loop {
            tokio::select!{
                msg = out_rv.recv() => {
                    if let Some(msg)=msg{
                        handler.read(ctx.as_ref(),msg).await;
                    }
                },
                msg=in_rv.recv()=>{
                    if let Some(msg)=msg{
                        let mut byte_buf = ByteBuf::new_with_capacity(self.read_size);
                        encoder.encode(ctx.as_ref(),msg,&mut byte_buf).await;
                        let mut buf = vec![0u8; byte_buf.readable_bytes()];
                        byte_buf.read_bytes(&mut buf);
                        w.write(&buf).await?;
                    }
                },
                _ = close.recv() => {
                    w.shutdown().await?;
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
        self.handler.close(ctx.as_ref()).await;
        result
    }


}



pub struct Bootstrap2<E, D, H, T,S>
where
    E: crate::encoder2::Encoder<T> + Send + 'static,
    D: crate::decoder2::Decoder<T> + Send + 'static,
    H: crate::handler2::Handler<T> + Send + 'static,
    T: Send + Sync + 'static,
    S: AsyncWriteExt + AsyncReadExt
{
    encoder: E,
    decoder: D,
    handler: H,
    ctx: crate::context2::Context<T>,
    in_rv: Receiver<T>,
    out_rv: Receiver<T>,
    r:ReadHalf<S>,
    w:WriteHalf<S>,
    close:Receiver<()>,
    read_size:usize,
    rate_limit:Option<TokenBucket>,
}

impl<E, D, H, T,S> Bootstrap2<E, D, H, T,S>
where
    E: crate::encoder2::Encoder<T> + Send + 'static,
    D: crate::decoder2::Decoder<T> + Send + 'static,
    H: crate::handler2::Handler<T> + Send + 'static,
    T: Send + Sync + 'static,
    S: AsyncWriteExt + AsyncReadExt
{
    pub fn build(stream: S,addr:SocketAddr, e: E, d: D, h: H) -> Self {
        let (in_tx, in_rv) = channel(1024);
        let (out_tx, out_rv) = channel(1024);
        let (close_tx, close_rv) = channel(128);
        let (r,w)=io::split(stream);
        
        Bootstrap2 {
            encoder: e,
            decoder: d,
            handler: h,
            ctx: crate::context2::Context::new(in_tx, out_tx,addr,close_tx),
            in_rv: in_rv,
            out_rv: out_rv,
            r:r,
            w:w,
            close:close_rv,
            read_size:READ_SIZE,
            rate_limit:None
        }
    }

    pub async fn set_rate_limit(&mut self,rate_limit:u64){
        let bucket = TokenBucket::new(rate_limit as f64, self.read_size*self.read_size);
        self.rate_limit.insert(bucket);
    }

    pub fn capacity(&mut self,size:usize){
        self.read_size=size;
    }

    async fn handle_run(&mut self)->std::io::Result<()>{
        let handler = &mut self.handler;
        let encoder = &mut self.encoder;

        let out_rv = &mut self.out_rv;
        let in_rv = &mut self.in_rv;

        let r=&mut self.r;
        let w=&mut self.w;
        

        let close=&mut self.close;
      
        let mut bf = ByteBuf::new_with_capacity(self.read_size);
        let mut buf = vec![0u8; self.read_size];
        let ctx = &mut self.ctx;

        handler.active(ctx).await;
        
        let limiter=&mut self.rate_limit;

        loop {
            tokio::select!{
                msg = out_rv.recv() => {
                    if let Some(msg)=msg{
                        handler.read(ctx,msg).await;
                    }
                },
                msg=in_rv.recv()=>{
                    if let Some(msg)=msg{
                        let mut byte_buf = ByteBuf::new_with_capacity(self.read_size);
                        encoder.encode(ctx,msg,&mut byte_buf);
                        let mut buf = vec![0u8; byte_buf.readable_bytes()];
                        byte_buf.read_bytes(&mut buf);
                        w.write(&buf).await?;
                    }
                },
                _ = close.recv() => {
                    w.shutdown().await?;
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
        let result=self.handle_run().await;
        self.handler.close(&mut self.ctx).await;
        result
    }


}