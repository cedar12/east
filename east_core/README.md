## 编码器
```rust
pub trait Encoder<T>{
  fn encode(&self,ctx:&Context<T>,msg:T,byte_buf:&mut ByteBuf);
}
```
> 编码器实现
```rust
pub struct MsgEncoder{}

impl Encoder<Msg> for MsgEncoder{
    fn encode(&self,ctx:&Context<Msg>,msg:Msg,byte_buf:&mut ByteBuf) {
        byte_buf.write_u8_be(0x86);
        byte_buf.write_u8_be(msg.msg_type as u8);
        byte_buf.write_u32_be(msg.data_len);
        byte_buf.write_bytes(&msg.data);
    }
}
```
## 解码器
```rust
#[async_trait]
pub trait Decoder<T>{
  async fn decode(&self,ctx:&Context<T>,byte_buf:&mut ByteBuf);
}
```
> 解码器实现
```rust

pub struct MsgDecoder{}

#[async_trait]
impl Decoder<Msg> for MsgDecoder{
    
    async fn decode(&self,ctx:&Context<Msg>,bf:&mut ByteBuf) {
        if bf.readable_bytes()<6{
            return
        }
        bf.mark_reader_index();
        let flag=bf.read_u8();
        if flag!=0x86{
            return
        }
        let t=bf.read_u8();

        let msg_type=TypesEnum::try_from(t).unwrap();
                
        let len=bf.read_u32_be() as usize;

        if bf.readable_bytes()<len{
            bf.reset_reader_index();
            return
        }
        let mut buf=vec![0u8;len];
        bf.read_bytes(&mut buf);
        bf.clean();
        let msg=Msg::new( msg_type, buf);
        // 通知处理器
        ctx.out(msg).await;
        self.decode(ctx, bf).await;
        
    }
    
}
```
## 处理器
```rust
#[async_trait]
pub trait Handler<T>{
  async fn active(&self,ctx:&Context<T>);
  async fn read(&self,ctx:&Context<T>,msg:T);
}
```
```rust
pub struct MsgHandler {}

#[async_trait]
impl Handler<Msg> for MsgHandler {
    async fn read(&self, ctx: &Context<Msg>, msg: Msg) {
        println!("handle read {:?}", msg);
        let m=Msg::new(TypesEnum::ProxyOpen, msg.data);
        ctx.write(m).await;
    }
    async fn active(&self, ctx: &Context<Msg>) {
        println!("active {:?}连接", ctx.addr());
    }
}
```
## 使用
```rust
#[tokio::main]
async fn main() -> Result<()> {
    let mut stream=TcpStream::connect("127.0.0.1:12345").await?;
    let addr=stream.peer_addr()?;
    Bootstrap::build(client,addr, MsgEncoder{}, MsgDecoder{}, MsgHandler{}).run().await?;
    Ok(())
}

```