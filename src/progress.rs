 pub trait ProgressDisplay {
     fn set_progress(&mut self, status: String, bytes_out: usize);
     fn update_status(&mut self, new_status: &str);
     fn add_bytes_written(&mut self, bytes_written: usize);
     fn set_size(&mut self, bytes_total: usize);
 }