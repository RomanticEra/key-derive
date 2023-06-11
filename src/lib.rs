#![doc = include_str!("../README.md")]
automod::dir!("src");

pub use config::CONFIG;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
