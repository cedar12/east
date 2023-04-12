use std::net::Ipv4Addr;

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Agent{
   pub bind_port:u16,
   #[serde(default = "default_host")]
   pub target_host:String,
   pub target_port:u16,
   #[serde(default = "defualt_max_rate")]
   pub max_rate:Option<u32>,
   #[serde(default = "default_whitelist")]
   pub whitelist:Vec<String>
}

fn default_host() -> String{
    String::from("127.0.0.1")
}

fn default_whitelist()->Vec<String>{
    vec![]
}

fn defualt_max_rate()->Option<u32>{
    None
}

impl Agent{
    pub fn match_addr(&self,addr:String)->bool{
        let addr_items:Vec<&str>=addr.split(":").collect();
        self.match_ip(addr_items[0].to_string())
    }
    pub fn match_ip(&self,ip:String)->bool{
        if self.whitelist.len()==0{
            return true
        }
        let target_ip:Ipv4Addr=ip.parse().unwrap();
        for (_,white) in self.whitelist.iter().enumerate(){
            if white==""{
                continue;
            }
            let v:Vec<&str>=white.split("/").collect();
            let mut prefix: u32=32;
            if v.len()==2{
                let r=v[1].parse::<u32>();
                match r{
                    Err(_)=>continue,
                    Ok(p)=>prefix = p
                }
            }
            let ip=v[0].parse::<Ipv4Addr>();
            match ip{
                Err(_)=>continue,
                Ok(ip)=>{
                    let rule = IpRule::new(ip,prefix);
                    if rule.matches(&target_ip) {
                        return true;
                    }
                }
            }
            
        }
        false
    }
}


struct IpRule {
    ip: u32,
    prefix_len: u32,
}

impl IpRule {
    fn new(ip: Ipv4Addr, prefix_len: u32) -> IpRule {
        let ip_arr = ip.octets();
        let ip = (ip_arr[0] as u32) << 24 | (ip_arr[1] as u32) << 16 | (ip_arr[2] as u32) << 8 | (ip_arr[3] as u32);
        IpRule {
            ip: ip,
            prefix_len: prefix_len,
        }
    }

    fn matches(&self, ip: &Ipv4Addr) -> bool {
        let dst: u32 = u32::from(*ip);
        let mask: u32 = !((1u32 << (32 - self.prefix_len)) - 1);
        (dst & mask) == (self.ip & mask)
    }
}