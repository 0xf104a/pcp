use std::os::unix::fs::MetadataExt;
use std::path::Path;
use async_trait::async_trait;
use crate::copy::DynBuffer;
use crate::reader::Reader;
use tokio::fs::File;
use regex::Regex;
pub struct FileReader{
    path: String,
}

#[async_trait]
impl Reader for FileReader{
    fn can_read(url: &str) -> bool where Self: Sized {
        let re = Regex::new(r"^(/[^/\\0]+)+/?$").unwrap();
        re.is_match(url)
    }
    fn new(url: &str) -> Self where Self: Sized {
        if !Self::can_read(url){
            panic!("Can not read url {url}");
        }
        FileReader{
            path: String::from(url),
        }
    }
    
    fn get_size(&self) -> usize {
        let metadata = std::fs::metadata(&self.path).expect("Can not read metadata");
        metadata.size() as usize
    }
    async fn read_chunk(&self, buffer: &mut DynBuffer, max_size: usize) -> usize {
        todo!()
    }
}