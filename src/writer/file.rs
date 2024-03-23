use std::path::Path;
use async_trait::async_trait;
use regex::Regex;
use tokio::fs::{File, OpenOptions};
use tokio::io::AsyncWriteExt;
use crate::copy::DynBuffer;
use crate::utils::runtime::tokio_block_on;
use crate::writer::Writer;

pub struct FileWriter{
    path: String,
    file: File,
}

#[async_trait]
impl Writer for FileWriter{
    fn new(url: &str) -> Self where Self: Sized {
        if !Self::can_write(url){
            //panic!("Can not write url {url}");
        }
        let open_coroutine = async {
            if Path::new(url).exists(){
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(url).await

            } else {
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .create_new(true)
                    .open(url).await
            }
        };

       FileWriter {
            path: String::from(url),
            file: tokio_block_on(open_coroutine).expect("Can not open file"),
        }
    }

    #[inline]
    fn can_write(url: &str) -> bool where Self: Sized {
        let re = Regex::new(r"^(/[^/\\0]+)+/?$|^[^/\\0]+$").unwrap();
        re.is_match(url)
    }

    #[inline]
    fn is_directory(url: &str) -> bool where Self: Sized {
        Path::new(url).is_dir()
    }
    
    #[inline]
    fn make_directory(url: &str) where Self: Sized {
        //println!("mkdir {:?}", url);
        std::fs::create_dir_all(Path::new(url)).expect("Can not create directory")
    }

    async fn write_chunk(&mut self, chunk: &DynBuffer, size: usize) {
        if chunk.len() == size {
            self.file.write(chunk).await.expect("Can not write file");
        } else { //chunk.len() > size
            self.file.write(&chunk[0..size]).await.expect("Can not write file");
        }
    }
}