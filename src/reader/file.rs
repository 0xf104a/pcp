use std::ffi::OsString;
use std::os::linux::raw::stat;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use async_trait::async_trait;
use crate::reader::Reader;
use tokio::fs::File;
use regex::Regex;
use tokio::io::AsyncReadExt;
use crate::utils::generic_iterator::GenericIterator;
use crate::utils::runtime::tokio_block_on;

pub struct FileReader{
    path: String,
    file: File,
}

struct DirectoryIteratorState{
    objects: Vec<OsString>,
    current_object: usize,
    full_path: OsString,
}

impl DirectoryIteratorState {
    pub fn new(path: String) -> DirectoryIteratorState{
        let path = Path::new(&path);
        let objects = if path.is_file(){
            path.iter().map(|x| x.to_os_string()).collect()
        } else {
            vec![OsString::from(&path)]
        };
        DirectoryIteratorState{
            objects,
            current_object: 0,
            full_path: OsString::from(&path),
        }
    }

    #[inline]
    pub fn len(&self) -> usize{
        self.objects.len()
    }

    pub fn next_object(&mut self) -> Option<OsString>{
        if self.current_object >= self.objects.len(){
            None
        } else {
            let result = Some(self.objects[self.current_object].clone());
            self.current_object += 1;
            result
        }
    }
}

struct DirectoryIterator {
    base_directory: String,
    state_stack: Vec<DirectoryIteratorState>,
}

impl DirectoryIterator{
    pub fn new(url: &str) -> DirectoryIterator{
        DirectoryIterator{
            base_directory: url.to_string(),
            state_stack: vec![DirectoryIteratorState::new(url.to_string())],
        }
    }
}

impl GenericIterator<String> for DirectoryIterator{
    fn internal_next(&mut self) -> Option<String> {
        if self.state_stack.len() == 0{
            return None;
        }
        let mut next_object = self.state_stack.last_mut().unwrap().next_object();
        while next_object.is_none() && self.state_stack.len() > 0{
            self.state_stack.pop();
            next_object = self.state_stack.last_mut().unwrap().next_object();
        }
        if next_object.is_none(){
            return None;
        }
        let path_os_string = next_object.unwrap();
        let path_string = path_os_string.to_str().unwrap().to_string();
        let path = Path::new(&path_os_string);
        if path.is_dir(){
            let mut state = DirectoryIteratorState::new(path_string);
            if state.len() == 0{
                return self.internal_next();
            }
            return Some(state.next_object().unwrap().to_str().unwrap().to_string());
        }
        Some(path_string)
    }
}



#[async_trait]
impl Reader for FileReader{
    #[inline]
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

    #[inline]
    fn is_directory(url: &str) -> bool where Self: Sized {
        std::path::Path::new(url).is_dir()
    }

    fn get_size(&self) -> usize {
        let metadata = std::fs::metadata(&self.path).expect("Can not read metadata");
        metadata.size() as usize
    }

    fn get_blocksize(&self) -> usize {
        let metadata = std::fs::metadata(&self.path).expect("Can not read metadata");
        metadata.blksize() as usize
    }

    #[inline]
    fn iter_directory(&self, url: &str) -> Box<dyn GenericIterator<String>>{
        Box::new(DirectoryIterator::new(url))
    }

    async fn read_chunk(&mut self, buffer: &mut [u8], _max_size: usize) -> usize {
        self.file.read(buffer).await.expect("Can not read file")
    }
}