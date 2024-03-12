use clap::Parser;

#[derive(Parser, Debug)]
pub(crate) struct Args {
    #[arg(short = 'r', long = "recursive", help = "Copy directories recursively")]
    pub recursive: bool,
    #[arg(long = "no-progress", help = "Do not show progress")]
    pub no_progress: bool,
    #[arg(help = "Source file/directories", required = true)]
    pub srcs: Vec<String>,
    #[arg(help = "Destination file/directory")]
    pub dest: String,
}