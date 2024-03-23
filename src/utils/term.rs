use termion::terminal_size;

#[inline]
pub fn flush() {
    let (size_rows, _) = terminal_size()
        .expect("Can not read terminal size");
    print!("\r");
    for _ in 0..size_rows - 1{
        print!(" ")
    }
    print!("\r");
}