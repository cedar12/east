use std::sync::Arc;

use crate::config;



#[test]
fn test_conf(){
    let conf=Arc::clone(&config::CONF);
    println!("{:?}",conf);

}