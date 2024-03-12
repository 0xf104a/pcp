use colored::Colorize;

use crate::progress::ProgressDisplay;
use crate::utils::get_time;

pub struct ConsoleProgress{
    bytes_total: usize,
    bytes_out: usize,
    status: String,
    last_precise_update: u128,
}

impl ConsoleProgress {
    fn print_progress(&mut self){
        let current_time = get_time();
        //Guard against too quick updates
        if current_time - self.last_precise_update < 40{
            return;
        }
        self.last_precise_update = current_time;
        //Guard against 0 sized things
        if self.bytes_total == 0{
            return;
        }
        print!("{}{} {}", "".clear(), &self.status, "[".bold());
        let num_bricks = 50 * self.bytes_out / self.bytes_total;
        for _ in 0..num_bricks{
            print!("{}", "#".green());
        }
        for _ in 0..(50 - num_bricks){
            print!(" ");
        }
        print!("{}{}\r", "".clear(), "]".bold());
    }
}

impl ProgressDisplay for ConsoleProgress {
    fn new() -> Self where Self: Sized {
        ConsoleProgress{
            bytes_out: 0,
            bytes_total: 0,
            status: String::from(""),
            last_precise_update: 0,
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