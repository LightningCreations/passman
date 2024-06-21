use common::suite::{AsymmetricCipherAlgorithm, DigestAlgorithm};
use uuid::Uuid;

pub struct UserRecord {
    userid: Uuid,
    address_digest_algorithm: DigestAlgorithm,
    address_hash: Vec<u8>,
    key_pair_algorithm: AsymmetricCipherAlgorithm,
    pubkey: Vec<u8>,
    sealed_priv_key: Vec<u8>,
    root_key_id: Uuid,
    root_object_id: Uuid,
}
