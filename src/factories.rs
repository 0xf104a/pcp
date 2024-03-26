use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::reader::Reader;
use crate::utils::generic_iterator::GenericIterator;
use crate::writer::Writer;

///
/// Allows access to static methods of Reader trait
///
#[derive(Clone)]
pub struct ReaderProxy{
    constructor: Box<fn(&str)-> Box<dyn Reader>>,
    can_read_fn: Box<fn(&str) -> bool>,
    is_directory_fn: Box<fn(&str) -> bool>,
    iter_directory_fn: Box<fn(&str) -> Box<dyn GenericIterator<String>>>,
    relative_path_fn: Box<fn(&str, &str) -> String>,
    dirname_fn: Box<fn(&str) -> String>,
    filename_fn: Box<fn(&str) -> String>,
    
}

///
/// Produces instances of Reader trait object via ReaderProxy
///
pub struct ReaderFactory{
    components: HashMap<String, ReaderProxy>
}

///
/// Allows access to static methods of Writer trait
///
#[derive(Clone)]
pub struct WriterProxy{
    constructor: Box<fn(&str)-> Box<dyn Writer>>,
    can_write_fn: Box<fn(&str) -> bool>,
    is_directory_fn: Box<fn(&str) -> bool>,
    make_directory_fn: Box<fn(&str)>,
    join_path_fn: Box<fn(&str, &str) -> String>,
}

impl ReaderProxy {
    pub fn from_type<T: Reader + 'static>() -> ReaderProxy{
        ReaderProxy{
            constructor: Box::new(|url| { Box::new(T::new(url)) }),
            can_read_fn: Box::new(T::can_read),
            is_directory_fn: Box::new(T::is_directory),
            iter_directory_fn: Box::new(T::iter_directory),
            relative_path_fn: Box::new(T::relative_path),
            dirname_fn: Box::new(T::dirname),
            filename_fn: Box::new(T::filename),
        }
    }

    #[inline]
    pub fn produce(&self, url: &str) -> Box<dyn Reader>{
        let fun = *self.constructor;
        fun(url)
    }
    
    #[inline]
    pub fn can_read(&self, url: &str) -> bool{
        let fun = *self.can_read_fn;
        fun(url)
    }
    
    #[inline]
    pub fn relative_path(&self, base: &str, path: &str) -> String{
        let fun = *self.relative_path_fn;
        fun(base, path)
    }
    
    #[inline]
    #[allow(dead_code)]
    pub fn dirname(&self, url: &str) -> String{
        let fun = *self.dirname_fn;
        fun(url)
    }

    #[inline]
    pub fn filename(&self, url: &str) -> String{
        let fun = *self.filename_fn;
        fun(url)
    }
    
    #[inline]
    pub fn iter_directory(&self, url: &str) -> Box<dyn GenericIterator<String>>{
        let fun = *self.iter_directory_fn;
        fun(url)
    }

    #[inline]
    pub fn is_directory(&self, url: &str) -> bool{
        let fun = *self.is_directory_fn;
        fun(url)
    }
}

impl WriterProxy{
    pub fn from_type<T: Writer + 'static>() -> WriterProxy {
        WriterProxy{
            constructor: Box::new(|url|{ Box::new(T::new(url)) }),
            can_write_fn: Box::new(T::can_write),
            is_directory_fn: Box::new(T::is_directory),
            make_directory_fn: Box::new(T::make_directory),
            join_path_fn: Box::new(T::join_path),
        }
    }
    
    #[inline]
    pub fn produce(&self, url: &str) -> Box<dyn Writer>{
        let fun = *self.constructor;
        fun(url)
    }
    
    #[inline]
    pub fn can_write(&self, url: &str) -> bool{
        let fun = *self.can_write_fn;
        fun(url)
    }
    
    #[inline]
    pub fn is_directory(&self, url: &str) -> bool{
        let fun = *self.is_directory_fn;
        fun(url)
    }
    
    #[inline]
    pub fn make_directory(&self, url: &str){
        let fun = *self.make_directory_fn;
        fun(url);
    }
    
    #[inline]
    pub fn join_path(&self, base: &str, path: &str) -> String{
        let fun = *self.join_path_fn;
        fun(base, path)
    }
}

pub struct WriterFactory{
    components: HashMap<String, WriterProxy>,
}


impl ReaderFactory {
    pub fn new() -> ReaderFactory{
        ReaderFactory{
            components: HashMap::new(),
        }
    }
    
    pub fn add_reader<T: Reader + 'static>(&mut self, key: &str){
        self.components.insert(key.to_string(), ReaderProxy::from_type::<T>());
    }

    pub fn get_reader_proxy(&self, url: &str) -> Option<ReaderProxy>{
        for (_, proxy) in self.components.iter(){
            if proxy.can_read(url){
                return Some(proxy.clone());
            }
        }
        None
    }
}

impl WriterFactory{
    pub fn new() -> WriterFactory{
        WriterFactory{
            components: HashMap::new()
        }
    }

    pub fn add_writer<T: Writer + 'static>(&mut self, key: &str){
        self.components.insert(key.to_string(), WriterProxy::from_type::<T>());
    }

    pub fn get_writer_proxy(&self, url: &str) -> Option<WriterProxy>{
        for (_, proxy) in self.components.iter(){
            if proxy.can_write(url){
                return Some(proxy.clone());
            }
        }
        None
    }
}

lazy_static! {
    pub static ref READER_FACTORY: Mutex<ReaderFactory> = Mutex::new(ReaderFactory::new());
    pub static ref WRITER_FACTORY: Mutex<WriterFactory> = Mutex::new(WriterFactory::new());
}

#[inline]
pub fn get_reader_proxy_for_url(url: &str) -> Option<ReaderProxy>{
    let result = READER_FACTORY.lock().unwrap().get_reader_proxy(url);
    result
}

#[inline]
pub fn get_writer_proxy_for_url(url: &str) -> Option<WriterProxy>{
    WRITER_FACTORY.lock().unwrap().get_writer_proxy(url)
}

