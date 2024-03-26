use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub(crate) struct Args {
    #[arg(short = 'r', long = "recursive", help = "Copy directories recursively")]
    pub recursive: bool,
    #[arg(short = 's', long = "no-progress", help = "Do not show progress")]
    pub no_progress: bool,
    #[arg(help = "Source file/directories", required = true)]
    pub srcs: Vec<String>,
    #[arg(help = "Destination file/directory")]
    pub dest: String,
    #[arg(long = "fail-fast", help = "Fail on first error")]
    pub fail_fast: bool,
    #[arg(long = "max-chunks-number", default_value = "1024", 
          help = "Maximum number of cached chunks of file stored in memory")]
    pub max_chunks_number: u64,
}