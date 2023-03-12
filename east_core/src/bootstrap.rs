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
    w:Arc<Mutex<WriteHalf<TcpStream>>>
}

impl<E, D, H, T> Bootstrap<E, D, H, T>
where
    E: Encoder<T> + Send + 'static,
    D: Decoder<T> + Send + 'static,
    H: Handler<T> + Send + 'static,
    T: Send + Sync + 'static,
{
    pub fn build(stream: TcpStream,addr:SocketAddr, e: E, d: D, h: H) -> Self {
        // stream.peer_addr();
        let (in_tx, in_rv) = channel(10);
        let (out_tx, out_rv) = channel(10);
        let (r,w)=io::split(stream);
        Bootstrap {
            encoder: Arc::new(Mutex::new(e)),
            decoder: d,
            handler: Arc::new(Mutex::new(h)),
            ctx: Arc::new(Context::new(in_tx, out_tx,addr)),
            in_rv: Arc::new(Mutex::new(in_rv)),
            out_rv: Arc::new(Mutex::new(out_rv)),
            r:Arc::new(Mutex::new(r)),
            w:Arc::new(Mutex::new(w))
        }
    }

    pub async fn run(&mut self) -> std::io::Result<()> {
        let handler = Arc::clone(&self.handler);
        let encoder = Arc::clone(&self.encoder);
        let ctx = Arc::clone(&self.ctx);
        let out_rv = Arc::clone(&self.out_rv);
        let in_rv = Arc::clone(&self.in_rv);

        let r=Arc::clone(&self.r);
        let w=Arc::clone(&self.w);
        
        handler.lock().await.active(ctx.as_ref()).await;
        tokio::spawn(async move {
            loop{
                let mut out = out_rv.lock().await;
                let msg = out.recv().await;
                if let Some(msg)=msg{
                    let h=handler.lock().await;
                    let h=h.read(ctx.as_ref(), msg);
                    h.await;
                }
            }
        });
        let ctx = Arc::clone(&self.ctx);
        tokio::spawn(async move {
            loop{
                let mut in_rv = in_rv.lock().await;
                let msg = in_rv.recv().await;
                if let Some(msg)=msg{
                    let mut byte_buf = ByteBuf::new_with_capacity(0);
                    encoder
                        .lock()
                        .await
                        .encode(ctx.as_ref(), msg, &mut byte_buf);
                    let mut buf = vec![0u8; byte_buf.readable_bytes()];
                    byte_buf.read_bytes(&mut buf);
                    // let ret=ctx.get_stream().write(&buf).await; 
                    w.lock().await.write(&buf).await;
                    // ctx.stream_write(&buf).await;
                    // stream.write(&buf).await;
                }
                
            }
        });
        // std::thread::spawn(move || loop {
        //     let in_rv = in_rv.lock().unwrap();
        //     let msg = in_rv.recv().unwrap();
        //     let mut byte_buf = ByteBuf::new_with_capacity(0);
        //     encoder
        //         .lock()
        //         .unwrap()
        //         .encode(ctx.as_ref(), msg, &mut byte_buf);
        //     let mut buf = vec![0u8; byte_buf.readable_bytes()];
        //     byte_buf.read_bytes(&mut buf);
        //     ctx.get_stream().lock().unwrap().write(&buf).unwrap();
        // });
        // self.read_run(read).await
        let mut bf = ByteBuf::new_with_capacity(0);
        let mut buf = [0u8; READ_SIZE];
        let ctx = &self.ctx;
        loop {
            let bytes_read = r.lock().await.read(&mut buf).await?;
            // let bytes_read = ctx.get_stream().await.write().unwrap().read(&mut buf).await?;
            if bytes_read == 0 {
                return Ok(());
            }
            // println!("n {}",bytes_read);
            bf.write_bytes(&buf[..bytes_read])?;
            self.decoder.decode(ctx, &mut bf).await;
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
