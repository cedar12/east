use crate::types::TypesEnum;

pub mod msg_encoder;
pub mod msg_decoder;
pub mod msg_handler;

pub const MSG_HEADER_LEN:usize=6;

#[derive(Debug)]
pub struct Msg{
    pub msg_type:TypesEnum,
    data_len:u32,
    pub data:Vec<u8>,
}

impl Msg{
    pub fn new(msg_type:TypesEnum,data:Vec<u8>)->Self{
        Msg{
            msg_type:msg_type,
            data_len:data.len() as u32,
            data:data,
        }
    }
}