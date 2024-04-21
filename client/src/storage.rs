use crate::macros::async_trait;

use async_std::io::Read;
use common::{
    error::{Error, Result},
    uuid::Uuid,
};

#[async_trait]
pub trait Storage {
    fn is_unlocked(&self) -> bool;
    fn authentication(&mut self) -> Box<dyn Authentication + '_>;
    async fn item(&mut self, id: Uuid) -> Result<Box<dyn Item + '_>>;
}

#[async_trait]
pub trait Authentication {
    fn is_unlocked(&self) -> bool;
    async fn authenticate(&mut self) -> Result<()>;
}

#[async_trait]
pub trait Item {
    async fn read(&mut self) -> Result<Box<dyn Read + '_>>;
}
