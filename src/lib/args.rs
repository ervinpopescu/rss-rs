use clap::Parser;

/// Command line arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// URL to get RSS from
    #[arg(short, long)]
    pub get_from_url: bool,

    /// File to get RSS from
    #[arg(short, long)]
    pub file: String,
    /// Number of pages provided to the RSS feed
    #[arg(short, long)]
    pub pages: Option<usize>,
}
