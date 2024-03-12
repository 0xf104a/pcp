pub mod file;

use async_trait::async_trait;

#[async_trait]
pub trait Reader{
    fn can_read(url: &str) -> bool where Self: Sized;
    fn new(url: &str) -> Self where Self: Sized;
    fn get_size(&self) -> usize;
    fn get_blocksize(&self) -> usize;
    async fn read_chunk(&mut self, buffer: &mut [u8], max_size: usize) -> usize;
}