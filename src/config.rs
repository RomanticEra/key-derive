//! 在我们的对话中，我们讨论了以下几个主题：
//!
//! 6. 如何从字符串中读取 `[u8; 32]` 类型的字节数组。
//! 7. 使用 `.env` 文件来存储配置变量。
//! 8. 如何加载 `.env` 文件中的变量并使其在应用程序中可用。
//! 9. 如何适配字符串类型的环境变量到 `[u8; 32]` 类型的数组。
//! 10. 如何使用 `serde::de::Error` 来创建自定义错误。
use crate::{
    derive_child::{derive_child_key, generate_public_key},
    prelude::*,
};

/// root_key_bytes 和 root_chain_code 是示例中用来表示
/// 根密钥（root key）和根链码（root chain code）的变量。
/// 在密钥派生过程中，根密钥和根链码用作起点，通过派生算法
/// 和索引号来生成子私钥和子链码。这样可以在根密钥的基础上派生出
/// 多个层级的子密钥，从而提供更灵活的密钥管理和派生能力。
#[derive(Deserialize, Debug)]
pub struct Config {
    /// root_key_bytes 是一个长度为 32 字节的字节数组，表示根密钥的值。在示例中，
    /// 我们使用了一个简单的值 [0x01; 32] 来表示根密钥，实际应用中应该使用真正的随机值。
    #[serde(deserialize_with = "deserialize_from_hex")]
    root_key_bytes: [u8; 32],

    /// root_chain_code 也是一个长度为 32 字节的字节数组，表示根链码的值。链码是密钥派生中的一部分，
    /// 用于增加密钥派生的安全性。在示例中，我们使用了一个简单的值 [0x02; 32] 来表示根链码，
    /// 实际应用中应该使用真正的随机值。
    #[serde(deserialize_with = "deserialize_from_hex")]
    root_chain_code: [u8; 32],
}

/// Access to parsed configuration.
pub static CONFIG: Lazy<Config> = Lazy::new(|| envy::from_env().expect("some env vars missing"));

fn deserialize_from_hex<'de, D>(deserializer: D) -> Result<[u8; 32], D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    let s: String = serde::Deserialize::deserialize(deserializer)?;

    s.into_bytes()
        .try_into()
        .map_err(|_| Error::custom("Invalid byte array size. Expected 32 bytes."))
}

impl Config {
    pub fn derive_child_key(&self, index: u32) -> (SecretKey, [u8; 32], PublicKey) {
        let Config {
            root_key_bytes,
            root_chain_code,
        } = *self;
        let (child_key, child_chain_code) =
            derive_child_key(&root_key_bytes, &root_chain_code, index);
        let child_public_key = generate_public_key(&child_key);
        (child_key, child_chain_code, child_public_key)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    use crate::CONFIG;
    #[test]
    fn show_config_with_root_public() {
        dotenv().ok();
        assert_debug_snapshot!(CONFIG.derive_child_key(0).2);
    }
}
