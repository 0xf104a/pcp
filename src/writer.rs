pub mod file;

use async_trait::async_trait;
use crate::copy::DynBuffer;

#[async_trait]
pub trait Writer{
    fn new(url: &str) -> Self where Self: Sized; 
    fn can_write(url: &str) -> bool where Self: Sized;
    async fn write_chunk(&mut self, chunk: &DynBuffer);
}