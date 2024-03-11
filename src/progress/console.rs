use crate::progress::ProgressDisplay;

pub struct ConsoleProgress{
    bytes_total: usize,
    bytes_out: usize,
    status: String,
}

impl ConsoleProgress {
    fn print_progress(&self){
        print!("{} [", &self.status);
        let num_bricks = 50 * self.bytes_out / self.bytes_out;
        for _ in 0..num_bricks{
            print!("#");
        }
        for _ in 0..(50 - num_bricks){
            print!(" ");
        }
        print!("]");
    }
}

impl ProgressDisplay for ConsoleProgress {
    fn new() -> Self where Self: Sized {
        ConsoleProgress{
            bytes_out: 0,
            bytes_total: 0,
            status: String::from(""),
        }
    }
    fn set_progress(&mut self, status: &str, bytes_out: usize) {
        self.status = String::from(status);
        self.bytes_out = bytes_out;
        self.print_progress();
    }

    fn update_status(&mut self, new_status: &str) {
        self.status = String::from(new_status);
        self.print_progress();
    }

    fn add_bytes_written(&mut self, bytes_written: usize) {
        self.bytes_out += bytes_written;
        self.print_progress();
    }

    fn set_size(&mut self, bytes_total: usize) {
        self.bytes_total = bytes_total;
        self.print_progress();
    }
}