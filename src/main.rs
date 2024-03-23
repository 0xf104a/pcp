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

pub fn copy_directory(source: &str, target: &str) -> bool{
    let mut target_path = target.to_string();
    let is_new_dir = if FileWriter::is_directory(target){
        target_path = FileWriter::join_path(&target_path, &FileReader::dirname(source));
        false
    } else {
        FileWriter::make_directory(target);
        true
    };
    for object in FileReader::iter_directory(source){
        let target_object = if is_new_dir{
            FileReader::relative_path(&FileReader::dirname(&source), &object)
        } else { 
            object.clone()
        };
        let reader = Box::new(FileReader::new(&object));
        let destination = FileWriter::join_path(&target_path, &target_object);
        //println!("target_path={}, object={}, dest={}", target_path, target_object, destination);
        let mut progress = Box::new(ConsoleProgress::new());
        let buffer_size = reader.get_blocksize();
        if FileReader::is_directory(&object){
            FileWriter::make_directory(&destination);
            continue;
        }
        let writer = Box::new(FileWriter::new(&destination));
        progress.set_progress(&*format!("{} -> {}", object, destination), 0);
        let coroutine = async move {
            copy_file::copy(reader, writer, progress, 1024, buffer_size).await
        };
        if !tokio_block_on(coroutine){
            return false;
        }
    }
    true
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
                copy_file::copy(reader, writer, progress, 1024, buffer_size).await
            };
            if !tokio_block_on(coroutine){
                exit(255);
            }
        } else {
            if !copy_directory(&source, &args.dest){
                exit(255);
            }
        }
    }
}
