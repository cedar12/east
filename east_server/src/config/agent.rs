use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Agent{
   pub bind_port:u16,
   #[serde(default = "default_host")]
   pub target_host:String,
   pub target_port:u16, 
   #[serde(default = "default_whitelist")]
   pub whitelist:Vec<String>
}

fn default_host() -> String{
    String::from("127.0.0.1")
}

fn default_whitelist()->Vec<String>{
    vec![]
}

impl Agent{
    pub fn match_addr(&self,addr:String)->bool{
        let mut ip=String::new();
        let addr_items:Vec<&str>=addr.split(":").collect();
        ip=addr_items[0].to_string();
        self.match_ip(ip)
    }
    pub fn match_ip(&self,ip:String)->bool{
        if self.whitelist.len()==0{
            return true
        }
        let target_ip_items:Vec<&str>=ip.split(".").collect();
        let mut result=true;
        for (_,white) in self.whitelist.iter().enumerate(){
            let ip_items:Vec<&str>=white.split(".").collect();
            if target_ip_items.len()!=ip_items.len(){
                break;
            }
            for (i,_) in target_ip_items.iter().enumerate(){
                if ip_items[i]=="*"{
                    continue;
                }
                if ip_items[i]!=target_ip_items[i]{
                    result=false;
                    break
                }
            }
            return true
        }
        result
    }
}