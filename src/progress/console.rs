use std::cmp::min;
use colored::Colorize;
use termion::terminal_size;

use crate::progress::ProgressDisplay;
use crate::utils::get_time;

const MAX_STATUS_WIDTH: u16 = 128;
const STATUS_WIDTH_FACTOR: f32 = 0.2;

/// Implements simple progress for console
pub struct ConsoleProgress{
    bytes_total: usize,
    bytes_out: usize,
    status: String,
    last_precise_update: u128,
}

fn pad_status(status: String, max_width: u16) -> String {
    let status_len = status.len();
    let max_width = max_width as usize;

    if status_len >= max_width {
        if max_width <= 3 {
            return "...".to_string();
        }
        let half_width = (max_width - 3) / 2;
        let end_start = status_len - half_width + (max_width - 3) % 2;
        return format!("{}...{}", &status[..half_width], &status[end_start..]);
    }

    format!("{:width$}", status, width = max_width)
}

impl ConsoleProgress {
    fn print_progress(&mut self){
        let current_time = get_time();
        //Guard against too quick updates
        if current_time - self.last_precise_update < 40{
            return;
        }
        //Guard against 0 sized things
        if self.bytes_total == 0{
            return;
        }

        //Scale our progressbar
        let (size_rows, _) = terminal_size()
            .expect("Can not read terminal size");
        let console_width = min(MAX_STATUS_WIDTH, size_rows);
        let status_width = (STATUS_WIDTH_FACTOR * (console_width as f32)) as u16;
        let progress_bar_width =
            console_width - status_width - 6; // 4 symbols for percentage, 2 for brackets
        //Store last update time
        self.last_precise_update = current_time;
        //Output everything
        print!("{}{} {}", "".clear(), &pad_status(self.status.clone(), status_width), "[".bold());
        let num_bricks = (progress_bar_width as usize * self.bytes_out) / self.bytes_total;
        //println!("{}", num_bricks);
        for _ in 0..num_bricks{
            print!("{}", "#".green());
        }
        for _ in 0..(progress_bar_width as usize - num_bricks){
            print!(" ");
        }
        print!("{}{} {}%\r", "".clear(), "]".bold(), 100 * self.bytes_out / self.bytes_total);
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

    #[inline]
    fn flush(&self) {
        crate::utils::term::flush();
    }
}