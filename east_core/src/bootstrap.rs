use tokio::net::TcpStream;
use tokio::sync::mpsc::Receiver;

use crate::byte_buf::ByteBuf;

use crate::handler::{Handler };
use crate::{context::Context, decoder::Decoder, encoder::Encoder};
use tokio::io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf, self};
use tokio::sync::{mpsc::channel,Mutex};
use std::net::SocketAddr;
use std::sync::Arc;

const READ_SIZE: usize = 1024;

pub struct Bootstrap<E, D, H, T>
where
    E: Encoder<T> + Send + 'static,
    D: Decoder<T> + Send + 'static,
    H: Handler<T> + Send + 'static,
    T: Send + Sync + 'static,
{
    encoder: Arc<Mutex<E>>,
    decoder: D,
    handler: Arc<Mutex<H>>,
    ctx: Arc<Context<T>>,
    in_rv: Arc<Mutex<Receiver<T>>>,
    out_rv: Arc<Mutex<Receiver<T>>>,
    r:Arc<Mutex<ReadHalf<TcpStream>>>,
    w:Arc<Mutex<WriteHalf<TcpStream>>>,
    close:Arc<Mutex<Receiver<()>>>,
}

impl<E, D, H, T> Bootstrap<E, D, H, T>
where
    E: Encoder<T> + Send + 'static,
    D: Decoder<T> + Send + 'static,
    H: Handler<T> + Send + 'static,
    T: Send + Sync + 'static,
{
    pub fn build(stream: TcpStream,addr:SocketAddr, e: E, d: D, h: H) -> Self {
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
        }
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
      
        let mut bf = ByteBuf::new_with_capacity(0);
        let mut buf = [0u8; READ_SIZE];
        let ctx = &self.ctx;

        let mut out = out_rv.lock().await;
        let mut in_rv = in_rv.lock().await;
        let mut close = close.lock().await;
        let mut r=r.lock().await;
        
        loop {
        // let msg = out.recv().await;
            tokio::select!{
                msg = out.recv() => {
                    if let Some(msg)=msg{
                        let h=handler.lock().await;
                        let h=h.read(ctx.as_ref(), msg);
                        h.await;
                    }
                },
                msg=in_rv.recv()=>{
                    if let Some(msg)=msg{
                        let mut byte_buf = ByteBuf::new_with_capacity(0);
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
                    println!("bootstrap close");
                    return Ok(())
                },
                n=r.read(&mut buf)=>{
                    match n{
                        Ok(0)=>{
                            // let h=self.handler.lock().await;
                            // h.close(ctx).await;
                            return Ok(())
                        },
                        Ok(n)=>{
                            if n == 0 {
                                return Ok(());
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
        match self.handle_run().await{
            Ok(())=>{
                // if ctx.is_run(){
                //     ctx.close().await;
                // }
                let h=self.handler.lock().await;
                h.close(ctx.as_ref()).await;
                Ok(())
            },
            Err(e)=>{
                let h=self.handler.lock().await;
                h.close(ctx.as_ref()).await;
                
                Err(e)
            }
        }
    }

    // async fn read_run(&mut self,mut read:ReadHalf<TcpStream) -> std::io::Result<()> {
    //     let mut bf = ByteBuf::new_with_capacity(0);
    //     let mut buf = [0u8; READ_SIZE];
    //     let ctx = &self.ctx;
    //     loop {
    //         let bytes_read = read.read(&mut buf).await?;
    //         // let bytes_read = ctx.get_stream().await.write().unwrap().read(&mut buf).await?;
    //         if bytes_read == 0 {
    //             return Ok(());
    //         }
    //         // println!("n {}",bytes_read);
    //         bf.write_bytes(&buf[..bytes_read])?;
    //         self.decoder.decode(ctx, &mut bf).await;
    //     }
    // }
}

unsafe impl<E, D, H, T> Send for Bootstrap<E, D, H, T>
where
    E: Encoder<T> + Send + 'static,
    D: Decoder<T> + Send + 'static,
    H: Handler<T> + Send + 'static,
    T: Send + Sync +'static,
{
}
