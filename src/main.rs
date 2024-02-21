#![feature(error_iter)]

pub mod app;
pub mod channel;
pub mod event;
pub mod tui;
pub mod ui;
pub mod update;

use crate::app::App;
use crate::channel::{get_channel, parse_channel};
use crate::tui::start_terminal;

use anyhow::Result;
use clap::Parser;
use regex::Regex;
use rss::Channel;
use std::{fs::File, io::BufReader};

/// Read RSS feed from Upwork
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// URL to get RSS from
    #[arg(short, long)]
    get_from_url: bool,

    /// File to get RSS from
    #[arg(short, long)]
    file: String,

    #[arg(short, long)]
    pages: Option<usize>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Args = Args::parse();
    let read_file: File = File::open(&args.file).unwrap();
    let local_channel = Channel::read_from(BufReader::new(read_file)).unwrap();
    let url = local_channel.link();
    let pages = args.pages.unwrap_or(2);
    match args.get_from_url {
        true => {
            let mut all_titles = Vec::<String>::new();
            let mut all_descriptions = Vec::<String>::new();
            for i in 1..pages {
                let remote_channel = get_channel(url.to_owned(), i).await;
                match remote_channel {
                    Ok(channel) => {
                        let (mut titles, mut descriptions) = parse_channel(&channel);

                        all_titles.append(&mut titles);
                        all_descriptions.append(&mut descriptions);
                    }
                    Err(e) => {
                        let sources = e.sources();
                        let last_source = sources.last().unwrap().to_string();
                        let re = Regex::new(r"^.*Temporary failure in name resolution$").unwrap();
                        println!("{:?}", last_source);
                        if re.is_match(&last_source) {
                            println!("No internet connection, please run without `--get-from-url` or connect to the internet.");
                        } else {
                            println!("Unknown error.")
                        }
                    }
                }
            }
            let mut app = App::new(all_titles, all_descriptions);
            start_terminal(&mut app)?;
        }
        false => {
            let (titles, descriptions) = parse_channel(&local_channel);
            let mut app = App::new(titles, descriptions);
            start_terminal(&mut app)?;
        }
    }
    Ok(())
}
