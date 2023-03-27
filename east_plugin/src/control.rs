
/// 代理转发控制
pub trait ProxyControl: Send + Sync{
  /// 启动代理转发端口
  fn start(&self,bind_port:u16);
  /// 停止代理转发端口
  fn stop(&self,bind_port:u16);
}

/// 代理端控制
pub trait AgentControl: Send + Sync {
  /// 断开代理端的连接
  fn close(&self,agent_id:String);

}