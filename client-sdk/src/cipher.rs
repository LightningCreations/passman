use crate::macros::async_trait;
use common::suite::*;

#[non_exhaustive]
pub enum CipherSuiteError {
    Unsupported,
}

pub type Result<T> = core::result::Result<T, CipherSuiteError>;

#[async_trait]
pub trait AsymmetricCipherSpi {
    async fn init(&mut self, alg: AsymmetricCipherAlgorithm) -> Result<()>;
}
