use std::process::exit;
use std::path::{Component, Path, PathBuf};

use clap::{arg, Parser};
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
use crate::progress::dummy::DummyProgress;

pub fn copy_directory(source: &str, target: &str){
    for object in FileReader::iter_directory(source){
        let src_path = Path::new(&object);
        let mut src_components = src_path.components();
        let src_dir = src_components.next().unwrap();
        let mut dst_path = PathBuf::new();
        dst_path.push(Path::new(target));
        for component in src_components{
            dst_path.push(component);
        }
        let mut dst_dir = dst_path.clone();
        dst_dir.pop();
        FileWriter::make_directory(&dst_dir.to_str().unwrap());
        let writer = Box::new(FileWriter::new(dst_path.to_str().unwrap()));
        let reader = Box::new(FileReader::new(&object));
        let mut progress = Box::new(ConsoleProgress::new()) as Box<dyn ProgressDisplay>;
        progress.set_progress(&*format!("{} -> {}", object, dst_path
            .to_str().unwrap()), 0);
        let buffer_size = reader.get_blocksize();
        let coroutine = async move {
            copy_file::copy(reader, writer, progress, 1024, buffer_size).await;
        };
        tokio_block_on(coroutine);
    }
}
fn main() {
    let args = Args::parse();
    let mut sources = Vec::<String>::new();
    for source in args.srcs {
        if FileReader::is_directory(&source) && !args.recursive{
            println!("{}{}: Is a directory, but recursive flag is not set, skipping", source.bold(), 
                     "".clear());
            continue;
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
        if !FileReader::is_directory(&source) {
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
        } else {
            copy_directory(&source, &args.dest);
        }
    }
}
