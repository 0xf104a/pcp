use std::time::{SystemTime, UNIX_EPOCH};

pub mod runtime;
pub mod generic_iterator;

#[inline]
pub fn get_time() -> u128{
    let start = SystemTime::now();
    start.duration_since(UNIX_EPOCH)
         .expect("Wrong system time").as_millis()
}