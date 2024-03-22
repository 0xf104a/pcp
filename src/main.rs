use std::process::exit;
use clap::Parser;
use colored::Colorize;

mod reader;
mod writer;
mod progress;
mod copy;
pub mod utils;
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
use crate::progress::dummy::DummyProgress;


fn main() {
    let args = Args::parse();
    let mut sources = Vec::<String>::new();
    for source in args.srcs {
        if FileReader::is_directory(&source) && !args.recursive{
            println!("{}{}: Is a directory, but recursive flag is not set, skipping", source.bold(), 
                     "".clear());
        }
        if FileReader::can_read(&source){
            sources.push(source)
        } else {
            println!("{}{}: Can not read file", source.bold(), "".clear());
        }
    }
    if sources.len() > 1 && !FileWriter::is_directory(&args.dest){
        println!("{}{}: Is not a directory", args.dest.bold(), "".clear());
        exit(255);
    }
    if !FileWriter::can_write(&args.dest){
        println!("{}{}: Can not write file", args.dest.bold(), "".clear());
        exit(255);
    }
    init_tokio();
    for source in sources{
        let writer = Box::new(FileWriter::new(&args.dest));
        let reader = Box::new(FileReader::new(&source));
        let mut progress = if args.no_progress {
            Box::new(DummyProgress::new()) as Box<dyn ProgressDisplay>
        } else {
            Box::new(ConsoleProgress::new()) as Box<dyn ProgressDisplay>
        };
        let buffer_size = reader.get_blocksize();
        progress.update_status(&*format!("{} -> {}", source, args.dest));
        let coroutine = async move { 
            copy_file::copy(reader, writer, progress, 1024, buffer_size).await;
        };
        tokio_block_on(coroutine);
    }
}
