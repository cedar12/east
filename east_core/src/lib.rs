pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod types;
pub mod byte_buf;
pub mod error;

pub mod context;
pub mod context2;
pub mod encoder;
pub mod encoder2;
pub mod handler;
pub mod handler2;
pub mod decoder;
pub mod decoder2;
pub mod bootstrap;

pub mod message;

pub mod token_bucket;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
