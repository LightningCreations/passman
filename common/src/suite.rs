use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Hash, PartialEq, Eq)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
pub enum SymmetricCipherAlgorithm {
    Aes128Gcm,
    Aes128Cbc,
    Aes256Gcm,
    Aes256Cbc,
    Chacha20,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Hash, PartialEq, Eq)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
pub enum AsymmetricCipherAlgorithm {
    Rsa2048,
    Rsa4096,
    Ec25519,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Hash, PartialEq, Eq)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
pub enum DigestAlgorithm {
    Sha256,
    Sha224,
    Sha384,
    Sha512,
    #[serde(rename = "sha512/256")]
    Sha512_256,
    #[serde(rename = "sha512/224")]
    Sha512_224,
    #[serde(rename = "sha3-256")]
    Sha3_256,
    #[serde(rename = "sha3-512")]
    Sha3_512,
}
