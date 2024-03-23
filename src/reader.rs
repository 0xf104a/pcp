pub mod file;

use std::todo;
use async_trait::async_trait;
use futures::Stream;

use crate::utils::generic_iterator::GenericIterator;

#[async_trait]
pub trait Reader{
    
    /// 
    /// Checks that file/directory with given URL can be read by this implementation of reader
    /// # Arguments 
    /// 
    /// * `url`: URL of file to check
    /// 
    /// returns: bool 
    /// 
    /// # Examples 
    /// 
    /// ```
    /// if !MyReader::can_read("scheme://path/to/file"){
    ///     // Use another reader
    /// }
    /// ```
    fn can_read(url: &str) -> bool where Self: Sized;
    
    /// 
    /// Creates new instance of reader from given URL
    /// # Arguments 
    /// 
    /// * `url`: URL of file to read
    /// 
    /// returns: Self 
    /// 
    /// # Examples 
    /// 
    /// ```
    /// 
    /// ```
    fn new(url: &str) -> Self where Self: Sized;
    
    /// 
    /// Checks that given URL is directory
    /// # Arguments 
    /// 
    /// * `url`: URL to check
    /// 
    /// returns: bool 
    /// 
    /// # Examples 
    /// 
    /// ```
    ///  assert!(!MyReader::is_directory("scheme://path/to/some/file"));
    /// ```
    fn is_directory(url: &str) -> bool where Self: Sized;
    
    ///
    /// Returns size of file
    ///
    /// returns: Self 
    ///
    /// # Examples 
    ///
    /// ```
    ///  let file_size = MyReader::new("scheme://path/to/file").get_blocksize();
    /// ```
    fn get_size(&self) -> usize;

    ///
    /// Gets blocksize for filesystem in which source file is stored
    /// # Arguments
    ///
    /// returns: usize: block size in source fs
    ///
    /// # Examples 
    ///
    /// ```
    ///  let io_block_size = MyReader::new("scheme://path/to/file").get_blocksize();
    /// ```
    fn get_blocksize(&self) -> usize;
    
    ///
    /// Creates directory iterator
    /// 
    fn iter_directory(url: &str) -> Box<dyn GenericIterator<String>> where Self: Sized;

    ///
    /// Gets a relative path from URL given argument
    ///
    /// # Arguments
    ///
    /// * `src_arg`: str containing source directory
    /// * `url`: str containing url from iter_directory
    ///
    /// # Examples
    /// ```
    /// let src_arg = "/tmp/foo";
    /// let url = "/tmp/foo/bar/file";
    ///
    /// assert_eq!(MyReader::relative_path2(src_arg, url), "foo/bar/file");
    ///```
    ///
    /// ```
    /// let src_arg = "ftp://my-server/some-dir/foo";
    /// let url = "ftp://my-server/some-dir/foo/bar/file";
    ///
    /// assert_eq!(MyFTPReader::relative_path2(src_arg, url), "foo/bar/file");
    ///```
    ///
    fn relative_path(src_arg: &str, url: &str) -> String where Self: Sized;
    
    ///
    /// Reads chunk from file. Chunk is limited by given 
    /// # Arguments
    ///
    /// * `buffer`: Mutable reference to buffer where data would be put
    /// * `max_size`: Maximum amount of bytes to read
    ///
    /// returns: usize: amount of bytes read
    ///
    /// # Examples 
    ///
    /// ```
    /// let reader = MyReader::new("scheme://path/to/file");
    /// let buffer = Vec::<u8>::new();
    /// for _ in 0..128 {
    ///     buffer.push(0)
    /// }
    /// let bytes_read = reader.read_chunk(&mut buffer, 128).await;
    /// ```
    async fn read_chunk(&mut self, buffer: &mut [u8], max_size: usize) -> usize;

}