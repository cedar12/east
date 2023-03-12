use tokio::net::TcpStream;
use tokio::sync::Mutex;

pub struct Connection{
    s:TcpStream,
    id:String,
}

impl Connection {
    fn new(s:TcpStream,id:String)->Self{
        Connection { s, id }
    }
}


pub struct Connections{
    conns:Mutex<Vec<Connection>>
}

impl Connections {
    pub fn new()->Self{
        Connections { conns: Mutex::new(Vec::new()) }
    }
    pub async fn push(&self,client:Connection){
        let mut conns=self.conns.lock().await;
        conns.push(client);
    }
}