use std::path::{Path, PathBuf};
use async_trait::async_trait;
use colored::Colorize;
use regex::Regex;
use tokio::fs::{File, OpenOptions};
use tokio::io::AsyncWriteExt;
use crate::copy::DynBuffer;
use crate::utils::runtime::tokio_block_on;
use crate::writer::Writer;

///
/// Implements standard writing for files in local FS
/// 
pub struct FileWriter{
    _path: String,
    file: File,
}

#[inline]
fn check_valid_url(url: &str) -> bool {
    let re = Regex::new(r"^(/?[\s\w'.-]+)+(/)?$").unwrap();
    re.is_match(url)
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
            _path: String::from(url),
            file: tokio_block_on(open_coroutine).expect("Can not open file"),
        }
    }
    
    fn can_write(url: &str) -> bool where Self: Sized {
        if !check_valid_url(url){
            return false;
        }
        let path = PathBuf::from(url);
        if path.is_dir() || url.bytes().last().unwrap() == '/' as u8{
            if !path.exists(){
                println!("{}{}: No such directory", url.bold().red(), "".clear());
                return false;
            }
            return true;
        }
        let dir = path.parent().unwrap();
        if dir.is_dir() || dir.to_str().unwrap() == ""{
            true
        } else {
            println!("{}{}: No such directory", url.bold().red(), "".clear());
            false
        }
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
    
    fn join_path(base: &str, path: &str) -> String where Self: Sized {
        let mut base_path = PathBuf::from(base);
        let relative_path = PathBuf::from(path);
        base_path.extend(relative_path.iter());
        base_path.into_os_string().into_string().unwrap()
    }

    async fn write_chunk(&mut self, chunk: &DynBuffer, size: usize) -> std::io::Result<usize> {
        if chunk.len() == size {
            self.file.write(chunk).await
        } else { //chunk.len() > size
            self.file.write(&chunk[0..size]).await
        }
    }
}