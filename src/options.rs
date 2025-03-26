use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Options {
    /// Path to the git repository
    /// If not specified, the current directory will be used
    #[arg(short, long, default_value = ".")]
    pub path: String,
    /// Branch to analyze
    /// If not specified, the current branch will be used
    #[arg(short, long, default_value = "HEAD")]
    pub branch: String,
    /// Use TUI interface
    #[arg(short, long)]
    pub tui: bool,
}
