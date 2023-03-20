
use std::net::IpAddr;



#[tokio::test]
async fn test_s(){
    let s=String::from("你好!");
    println!("{:?}",s.as_bytes());
    let a:IpAddr=String::from("www.baidu.com").parse().unwrap();
    println!("{:?}",a);
}