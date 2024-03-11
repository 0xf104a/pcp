use async_trait::async_trait;
use regex::Regex;
use tokio::fs::File;
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
            panic!("Can not read url {url}");
        }
        let open_coroutine = async {
            File::open(url).await
        };

       FileWriter {
            path: String::from(url),
            file: tokio_block_on(open_coroutine).expect("Can not open file"),
        }
    }

    fn can_write(url: &str) -> bool where Self: Sized {
        let re = Regex::new(r"^(/[^/\\0]+)+/?$").unwrap();
        re.is_match(url)
    }

    async fn write_chunk(&mut self, chunk: &DynBuffer) {
        self.file.write(chunk).await.expect("Can not write file");
    }
}