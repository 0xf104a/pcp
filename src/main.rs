use crate::copy::{copy_directory, copy_file};
use std::process::exit;

use clap::Parser;
use colored::Colorize;

mod reader;
mod writer;
mod progress;
mod copy;
mod utils;
mod arguments;
mod factories;

use crate::arguments::Args;
use crate::factories::{get_reader_proxy_for_url, get_writer_proxy_for_url};
use crate::reader::register_readers;
use crate::writer::register_writers;
use crate::utils::runtime::init_tokio;


fn main() {
    register_readers();
    register_writers();
    let args = Args::parse();
    let cloned_args = args.clone();
    let mut sources = Vec::<String>::new();
    let writer_proxy = get_writer_proxy_for_url(&args.dest);
    if writer_proxy.is_none(){
        println!("{}: {}No writer for URL", args.dest.red().bold(), "".clear());
        exit(255);
    }
    let writer_proxy = writer_proxy.unwrap();
    for source in args.srcs {
        let reader_proxy = get_reader_proxy_for_url(&source);
        if reader_proxy.is_none(){
            println!("{}: {}No reader for URL", source.red().bold(), "".clear());
            if args.fail_fast{
                exit(255);
            }
            continue;
        }
        let reader_proxy = reader_proxy.unwrap();
        if reader_proxy.is_directory(&source) && !args.recursive{
            println!("{}{}: Is a directory, but recursive flag is not set, skipping", source.bold(), 
                     "".clear());
            continue;
        }
        if reader_proxy.can_read(&source){
            sources.push(source)
        } else {
            println!("{}{}: Can not read source", source.bold(), "".clear());
        }
    }
    if sources.len() > 1 && !writer_proxy.is_directory(&args.dest){
        println!("{}{}: Is not a directory", args.dest.bold(), "".clear());
        exit(255);
    }
    init_tokio();
    for source in sources{
        let proxy = get_reader_proxy_for_url(&source).unwrap();
        if proxy.is_directory(&source) {
            if !copy_directory(&source, &args.dest, &cloned_args) && args.fail_fast{
                exit(255);
            }
        } else {
            if !copy_file(&source, &args.dest, &cloned_args) && args.fail_fast{
                exit(255);
            }
        }
    }
}
