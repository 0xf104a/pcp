///
/// Simplified iterator for primitive types
///
pub trait GenericIterator<T>{
    ///
    /// Produce next object or tell that sequence is stopped
    /// 
    fn internal_next(&mut self) -> Option<T>;
}

impl<T> Iterator for dyn GenericIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.internal_next() as Option<Self::Item>
    }
}