//! 我们讨论了以下几个主题：
//! 1. 使用一个 Crate 来描述整个项目的配置文件的路径、多个数据库和公钥。
//! 2. 是否应该保存用户私钥。
//! 3. 如何安全地保存用户私钥。
//! 4. 密钥派生的概念和过程。
//! 5. 如何使用一个 Crate 来实现密钥派生。
//! 6. root_key_bytes 和 root_chain_code是什么. [./config.rs]
//! 7.index的可能值是什么?,有什么用?
use crate::prelude::*;

/// 代码演示了如何使用HMAC-SHA512作为密钥派生函数从父密钥派生子密钥。
/// 它接受父键、链码和索引作为输入，并返回派生的子键和链码。
/// derive_child_key函数用提供的链代码初始化HMAC-SHA512实例。
/// 然后使用serialize_private_key函数用序列化的私钥和索引更新HMAC。
/// * 在完成HMAC计算后，它提取前32个字节作为派生密钥，其余32个字节作为子链代码。
pub(crate) fn derive_child_key(
    parent_key: &[u8],
    chain_code: &[u8],
    // 索引值为0：表示根密钥（root key）。
    // 索引值为1：表示根密钥的第一个子密钥。
    // 索引值为2：表示根密钥的第二个子密钥。
    index: u32,
) -> (SecretKey, [u8; 32]) {
    let parent_key = &SecretKey::from_slice(parent_key).unwrap();
    // from chain_code
    let mut hmac = Hmac::<Sha512>::new_from_slice(chain_code).expect("HMAC initialization failed");
    // chain_code+parent_key
    hmac.update(&serialize_private_key(parent_key, index));

    let result = hmac.finalize();
    let hmac_output = result.into_bytes();

    let secret_key = SecretKey::from_slice(&hmac_output[0..32]).expect("Invalid secret key");
    let child_chain_code: [u8; 32] = hmac_output[32..].try_into().expect("Invalid chain code");

    (secret_key, child_chain_code)
}

pub(crate) fn serialize_private_key(key: &SecretKey, index: u32) -> Vec<u8> {
    let mut serialized_key = Vec::new();
    serialized_key.extend_from_slice(&key[..]);
    serialized_key.extend_from_slice(&index.to_be_bytes());
    serialized_key
}

pub(crate) fn generate_public_key(private_key: &SecretKey) -> PublicKey {
    let secp = Secp256k1::new();
    PublicKey::from_secret_key(&secp, private_key)
}

// Example usage
#[test]
fn show_child_key_1() {
    use crate::CONFIG;
    dotenv().ok();
    assert_debug_snapshot!(CONFIG.derive_child_key(1).2);
}
