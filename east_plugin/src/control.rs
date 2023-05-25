/// 代理转发控制
pub trait ProxyControl: Send + Sync{
  /// 启动代理转发端口
  fn start(&self,id:String,bind_port:u16);
  /// 停止代理转发端口
  fn stop(&self,id:String,bind_port:u16);
}

/// 代理端控制
pub trait AgentControl: Send + Sync {
  /// 断开代理端的连接
  fn close(&self,agent_id:String);
  /// 代理端是否在线
  fn is_online(&self,agent_id:String)->bool;
  /// 发送文件至代理端
  fn send_file(&self,agent_id:String,path:String,target:String);
}
