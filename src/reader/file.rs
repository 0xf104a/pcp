use std::os::unix::fs::MetadataExt;
use async_trait::async_trait;
use crate::reader::Reader;
use tokio::fs::File;
use regex::Regex;
use tokio::io::AsyncReadExt;
use crate::utils::runtime::tokio_block_on;

pub struct FileReader{
    path: String,
    file: File,
}

#[async_trait]
impl Reader for FileReader{
    fn can_read(url: &str) -> bool where Self: Sized {
        let re = Regex::new(r"^(/[^/\\0]+)+/?$|^[^/\\0]+$").unwrap();
        re.is_match(url)
    }
    fn new(url: &str) -> Self where Self: Sized {
        if !Self::can_read(url){
            panic!("Can not read url {url}");
        }
        let open_coroutine = async {
            File::open(url).await
        };
        
        FileReader{
            path: String::from(url),
            file: tokio_block_on(open_coroutine).expect("Can not open file"),
        }
    }

    fn get_size(&self) -> usize {
        let metadata = std::fs::metadata(&self.path).expect("Can not read metadata");
        //println!("size={}", metadata.size());
        metadata.size() as usize
    }

    fn get_blocksize(&self) -> usize {
        let metadata = std::fs::metadata(&self.path).expect("Can not read metadata");
        metadata.blksize() as usize
    }

    async fn read_chunk(&mut self, buffer: &mut [u8], _max_size: usize) -> usize {
        self.file.read(buffer).await.expect("Can not read file")
    }
}