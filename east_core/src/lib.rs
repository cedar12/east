pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod types;
pub mod byte_buf;
pub mod error;

pub mod context;
pub mod encoder;
pub mod handler;
pub mod decoder;
pub mod bootstrap;

pub mod message;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
