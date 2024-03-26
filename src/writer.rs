pub mod file;

use async_trait::async_trait;
use crate::copy::DynBuffer;
use crate::factories::WRITER_FACTORY;

#[async_trait]
pub trait Writer{
    fn new(url: &str) -> Self where Self: Sized; 
    fn can_write(url: &str) -> bool where Self: Sized;
    fn is_directory(url: &str) -> bool where Self: Sized;
    fn make_directory(url: &str) where Self: Sized;
    
    ///
    /// Joins base path with relative path
    /// 
    /// # Arguments
    /// * `base` path to base directory
    /// * `path` relative path in directory
    /// 
    /// # Examples
    /// ```
    /// let base = "foo/bar";
    /// let relative = "folder/file";
    /// 
    /// assert_eq!(MyWriter::join_path(base, relative), "foo/bar/folder/file);
    /// ```
    ///
    /// ```
    /// let base = "ftp://my-server/foo/bar";
    /// let relative = "folder/file";
    ///
    /// assert_eq!(MyWriter::join_path(base, relative), "ftp://my-server/foo/bar/folder/file);
    /// ```
    fn join_path(base: &str, path: &str) -> String where Self: Sized;
    async fn write_chunk(&mut self, chunk: &DynBuffer, size: usize) -> std::io::Result<usize>;
}

//FUTURE: refactor this to be done via macros
pub fn register_writers(){
    let mut factory = WRITER_FACTORY.lock().unwrap();
    factory.add_writer::<crate::writer::file::FileWriter>("file");
}