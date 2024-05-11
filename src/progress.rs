pub mod console;
pub mod dummy;

pub trait ProgressDisplay {
     fn new() -> Self where Self: Sized;
     fn set_progress(&mut self, status: &str, bytes_out: usize);
     #[allow(dead_code)]
     fn update_status(&mut self, new_status: &str);
     fn add_bytes_written(&mut self, bytes_written: usize);
     fn set_size(&mut self, bytes_total: usize);
     fn flush(&self);
 }