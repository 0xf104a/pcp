use clap::Parser;

#[derive(Parser, Debug)]
pub(crate) struct Args{
    #[arg(short='r', long="recursive", help="Copy directories recursively")]
    recursive: bool,
    #[arg(long="--no-progress", help="Do not show progress")]
    no_progress: bool,
    #[arg(help="Source file/directories")]
    srcs: Vec<String>,
    #[arg()]
    dest: Vec<String>
}