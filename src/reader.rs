mod file;

use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use crate::copy::DynBuffer;

#[async_trait]
pub trait Reader{
    fn can_read(url: &str) -> bool where Self: Sized;
    fn new(url: &str) -> Self where Self: Sized;
    fn get_size(&self) -> usize;
    async fn read_chunk(&mut self, buffer: &mut DynBuffer, max_size: usize) -> usize;
}