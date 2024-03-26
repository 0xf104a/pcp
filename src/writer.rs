pub mod file;

use async_trait::async_trait;
use crate::copy::DynBuffer;
use crate::factories::WRITER_FACTORY;

#[async_trait]
pub trait Writer{
    /// Creates writer instance for URL
    fn new(url: &str) -> Self where Self: Sized;

    ///
    /// Checks that Writer can write by URL
    ///
    /// # Arguments
    /// * url: &str: URL to check
    ///
    /// # Returns
    /// bool:
    ///   * true if URL is writable
    ///   * false otherwise
    fn can_write(url: &str) -> bool where Self: Sized;
    
    ///
    /// Checks that given URL is directory
    /// 
    /// # Arguments
    /// * url: &str: URL to check
    /// 
    /// # Returns
    /// bool:
    ///   * true if URL is directory
    ///   * false otherwise
    fn is_directory(url: &str) -> bool where Self: Sized;
    
    ///
    /// Creates given path(equivalent to mkdir -p)
    /// 
    /// # Arguments
    /// * url: &str: path to create
    /// 
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
    
    ///
    /// Writes chunk of given size and returns whether write was successful
    /// 
    /// # Arguments
    /// 
    /// * chunk: &DynBuffer: reference to buffer with chunk data
    /// * size: usize: number of bytes to write from chunk
    /// 
    /// # Example
    /// ```
    /// let mut writer = MyWriter("scheme://path/to/file");
    /// let buffer = DynBuffer::make_buffer(42);
    /// 
    /// writer.write_chunk(&buffer, 42);
    /// ```
    async fn write_chunk(&mut self, chunk: &DynBuffer, size: usize) -> std::io::Result<usize>;
}

//FUTURE: refactor this to be done via macros
pub fn register_writers(){
    let mut factory = WRITER_FACTORY.lock().unwrap();
    factory.add_writer::<crate::writer::file::FileWriter>("file");
}