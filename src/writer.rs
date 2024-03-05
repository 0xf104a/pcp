use async_trait::async_trait;
use crate::copy::DynBuffer;

#[async_trait]
pub trait Writer{
    fn can_write(&self, dest: &str) -> bool;
    fn set_destination(&mut self, dest: &str);
    async fn write_chunk(&mut self, chunk: &DynBuffer);
}