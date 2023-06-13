use std::{sync::Arc, path::Path, fs};

use east_core::{handler::Handler, message::Msg, context::Context, types::TypesEnum, byte_buf::ByteBuf, bootstrap2::Bootstrap, handler2::HandlerMut};
use tokio::{net::TcpStream, spawn, time, task::JoinHandle, sync::{broadcast::{Sender,Receiver}, self}, fs::{File, OpenOptions}, io::{BufWriter, AsyncWriteExt}};

use crate::{proxy::{proxy_encoder::ProxyEncoder, proxy_decoder::ProxyDecoder, proxy_handler::ProxyHandler, self}, config, key};


pub struct AgentHandler {
  tx:Sender<()>
}

impl AgentHandler{
  pub fn new()->Self{
    let (tx,_)=sync::broadcast::channel(1);
    AgentHandler { tx: tx}
  }
}

#[async_trait::async_trait]
impl Handler<Msg> for AgentHandler {
    async fn read(&mut self, ctx: &Context<Msg>, msg: Msg) {
        // println!("read len {:?}", msg.data.len());
        match msg.msg_type{
          TypesEnum::Auth=>{
            log::info!("启动发送心跳线程");
            let ctx=ctx.clone();
            let mut sub=self.tx.subscribe();
            spawn(async move{
              loop {
                  time::sleep(time::Duration::from_millis(10000)).await;
                  let result=sub.try_recv();
                  if result.is_ok(){
                    log::info!("退出发送心跳线程");
                    return
                  }
                  let msg=Msg::new(TypesEnum::Heartbeat, vec![]);
                  ctx.write(msg).await;
              }
            });
          },
          TypesEnum::ProxyOpen=>{
            spawn(proxy_open(msg,ctx.clone()));
          },
          TypesEnum::ProxyForward=>{
            proxy_forward(msg).await;
          },
          TypesEnum::ProxyClose=>{
            let mut bf=ByteBuf::new_from(&msg.data);
            let id=bf.read_u64_be();
            // proxy::ProxyMap.lock().await.remove(&id);
            // let map=proxy::ProxyMap.lock().await;
            // println!("agent close {} {:?} ",id, map);
            match proxy::ProxyMap.read().await.get(&id){
              Some(ctx)=>{
                ctx.close().await;
                // proxy::ProxyMap.lock().await.remove(&id);
                log::info!("agent close {} ",id);
              }, 
              None=>{
                log::warn!("{} 不存在",id)
              }
            }
            
          },
          TypesEnum::Heartbeat=>{
          },
          TypesEnum::FileInfo=>{
            let mut bf=ByteBuf::new_from(&msg.data);
            let file_size=bf.read_u64_be();
            let file_path=bf.read_string(msg.data.len()-8);
            let fp=file_path.clone();
            if Path::new(fp.clone().as_str()).exists() && fs::metadata(fp.clone()).unwrap().is_file() {
                println!("File {} exists.", fp);
                let msg=Msg::new(TypesEnum::FileInfoAsk, format!("{}文件存在",fp).as_bytes().to_vec());
                ctx.write(msg).await;
                return
            }
            log::info!("创建文件{}", fp);
            if let Err(e)=File::create(fp).await{
              let msg=Msg::new(TypesEnum::FileInfoAsk, e.to_string().as_bytes().to_vec());
              ctx.write(msg).await;
              return
            }
            let fi=FileInfo{size:file_size,path:file_path};
            ctx.set_attribute("fileinfo".into(), Box::new(fi.clone())).await;
            let msg=Msg::new(TypesEnum::FileInfoAsk, vec![]);
            ctx.write(msg).await;
            log::info!("接收文件信息: {:?}", fi);
          },
          TypesEnum::File=>{
            log::info!("接收文件数据大小: {}",msg.data.len());
            let file_info=ctx.get_attribute("fileinfo".into()).await;
            let file_info=file_info.lock().await;
            if let Some(info)=file_info.downcast_ref::<FileInfo>(){
              log::info!("接收文件信息: {:?}",info);
              let ctx=ctx.clone();
              let info=info.clone();
              spawn(async move{
                if let Err(e)=append_file(info.path.as_str(),msg.data).await{
                  log::error!("{}",e);
                  let msg=Msg::new(TypesEnum::FileAsk, e.to_string().as_bytes().to_vec());
                  ctx.write(msg).await;
                  return
                }
              });
              
              //let msg=Msg::new(TypesEnum::FileAsk, vec![]);
              //ctx.write(msg).await;
              return
            }
            let msg=Msg::new(TypesEnum::FileAsk, "未接收到文件信息".as_bytes().to_vec());
            ctx.write(msg).await;
          }
          _=>{}
        }

    }
    async fn active(&mut self, ctx: &Context<Msg>) {
        log::info!("已连接 {:?}", ctx.addr());
        let conf=Arc::clone(&config::CONF);
        let id=conf.id.clone();
        match key::encrypt(id){
          Ok(data)=>{
            let msg=Msg::new(TypesEnum::Auth,data);
            ctx.write(msg).await;
          },
          Err(e)=>{
            log::error!("公钥加载失败{:?}",e);
            ctx.close().await;
          }
        }
        
    }
    async fn close(&mut self, ctx: &Context<Msg>) {
        log::info!("关闭 {:?} ", ctx.addr());
        let _=self.tx.send(());
        // let mut map=proxy::ProxyMap.lock().await;
        let mut map=proxy::ProxyMap.write().await;
        for (_,v) in map.iter(){
          v.close().await;
        }
        map.clear();
    }
}

async fn append_file(file_path:&str,data:Vec<u8>)->std::io::Result<()>{
  let mut options = OpenOptions::new();
  options.append(true).create(true);
  let file = options.open(file_path).await?;
  let mut writer = BufWriter::new(file);
  writer.write_all(&data).await?;
  writer.flush().await?;
  Ok(())
}

async fn proxy_open(msg:Msg,ctx: Context<Msg>){
  let bytes=msg.data;
  let mut bf=ByteBuf::new_from(&bytes[..]);
  // let i1=bf.read_u8();
  // let i2=bf.read_u8();
  // let i3=bf.read_u8();
  // let i4=bf.read_u8();
  let host=bf.read_string_with_u8_be_len();
  let port = bf.read_u16_be();
  let addr=format!("{}:{}",host,port).to_string();
  let id=bf.read_u64_be();
  //log::info!("fid->{},ip->{}",id,addr);
  let stream=TcpStream::connect(addr).await;
  match stream{
    Ok(stream)=>{
      let addr=stream.peer_addr().unwrap();
      log::info!("代理连接{}",addr);
      let mut boot=Bootstrap::build(stream, addr, ProxyEncoder{}, ProxyDecoder{}, ProxyHandler{ctx: ctx.clone(),id:id});
      // boot.set_rate_limiter(east_core::token_bucket::TokenBucket::new(1024,1024)).await;
      let result=boot.run().await;
      if let Err(e)=result{
        log::error!("{:?}",e);
        let mut bf=ByteBuf::new_with_capacity(0);
        bf.write_u64_be(id);
        let msg=Msg::new(TypesEnum::ProxyClose, bf.available_bytes().to_vec());
        ctx.write(msg).await;
      }
    },
    Err(e)=>{
      log::error!("{:?}",e);
      let mut bf=ByteBuf::new_with_capacity(0);
      bf.write_u64_be(id);
      let msg=Msg::new(TypesEnum::ProxyClose, bf.available_bytes().to_vec());
      ctx.write(msg).await;
    }
  }
  
}

async fn proxy_forward(msg:Msg){
  let bytes=msg.data;
  let mut bf=ByteBuf::new_from(&bytes[..]);
  let id=bf.read_u64_be();
  let mut buf=vec![0u8;bf.readable_bytes()];
  bf.read_bytes(&mut buf);
  // println!("forward len {}:{} proxyMap {:?}",bytes.len(),buf.len(),proxy::ProxyMap.lock().await);
  // match proxy::ProxyMap.lock().await.get(&id){ 
  match proxy::ProxyMap.read().await.get(&id){ 
    Some(ctx)=>{
      ctx.write(buf.to_vec()).await;
    },
    None=>{
      log::warn!("无对应id连接{}",id);
    }
  };
}


#[derive(Debug,Clone)]
struct FileInfo{
  pub size:u64,
  pub path:String
}


