#![doc = include_str!("../README.md")]
automod::dir!("src");

pub use config::hex_to_slice as decode;
pub use config::CONFIG;
pub use hex::encode;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
