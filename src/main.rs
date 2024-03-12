use std::process::exit;
use clap::Parser;
use colored::Colorize;

mod reader;
mod writer;
mod progress;
mod copy;
mod utils;
mod arguments;

use crate::arguments::Args;
use crate::writer::file::FileWriter;
use crate::reader::file::FileReader;
use crate::progress::console::ConsoleProgress;
use crate::reader::Reader;
use crate::writer::Writer;
use crate::progress::ProgressDisplay;
use crate::utils::runtime::{init_tokio, tokio_block_on};
use crate::copy as copy_file;


fn main() {
    let args = Args::parse();
    let mut sources = Vec::<String>::new();
    for source in args.srcs {
        if FileReader::can_read(&source){
            sources.push(source)
        } else {
            println!("{}{}: Can not read file", source.bold(), "".clear());
        }
    }
    if !FileWriter::can_write(&args.dest){
        println!("{}{}: Can not write file", args.dest.bold(), "".clear());
        exit(255);
    }
    init_tokio();
    for source in sources{
        let writer = Box::new(FileWriter::new(&args.dest));
        let reader = Box::new(FileReader::new(&source));
        let progress = Box::new(ConsoleProgress::new());
        let buffer_size = reader.get_blocksize();
        let coroutine = async move { 
            copy_file::copy(reader, writer, progress, 1024, buffer_size).await;
        };
        tokio_block_on(coroutine);
    }
}
