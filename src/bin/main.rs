#[macro_use]
extern crate log;

use anyhow::Result;
use clap::Parser;
use reqwest::Client;
use rss::Channel;

use std::{fs::File, io::BufReader};
use storyship_lib::{
    app::App,
    args::Args,
    channel::{get_channel, handle_errors, parse_channel},
    consts::DEFAULT_PAGES,
    tui::start_terminal,
};

#[doc(hidden)]
#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();
    let read_file = File::open(&args.file)?;
    let local_channel = Channel::read_from(BufReader::new(read_file))?;
    let pages = args.pages.unwrap_or(DEFAULT_PAGES);
    let (titles, descriptions) = if args.get_from_url {
        log::info!("reading from remote RSS channel");
        let mut all_titles = Vec::<String>::new();
        let mut all_descriptions = Vec::<String>::new();
        let client = Client::new();

        for i in 1..=pages {
            let url = local_channel.link().to_string();
            let remote_channel = get_channel(url, i, &client).await;
            match remote_channel {
                Ok(remote_channel) => {
                    let (mut remote_titles, mut remote_descriptions) =
                        parse_channel(&remote_channel);
                    all_titles.append(&mut remote_titles);
                    all_descriptions.append(&mut remote_descriptions);
                }
                Err(e) => {
                    handle_errors(e);
                    println!();
                }
            }
        }

        (all_titles, all_descriptions)
    } else {
        if pages != 1 {
            warn!(
                "-p/--pages cannot be used without --g/--get_from_url, reverting to default {}",
                DEFAULT_PAGES
            )
        }
        parse_channel(&local_channel)
    };

    let mut app = App::new(titles, descriptions);
    start_terminal(&mut app)?;

    Ok(())
}
