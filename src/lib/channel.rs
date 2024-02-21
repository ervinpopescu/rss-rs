use crate::consts::PAGING_MAX;
use html_escape::decode_html_entities;
use log::error;
use regex::Regex;

use reqwest::{Client, Url};
use rss::Channel;
use std::{collections::HashMap, error::Error};
use url_escape::decode;
/// get a remote channel
pub async fn get_channel(
    url: String,
    page: usize,
    client: &Client,
) -> Result<Channel, Box<dyn Error>> {
    let url = decode(&url).to_string();
    let url = url
        .replace("&amp;", "&")
        .replace("?amp;", "?")
        .replace("amp;", "&");
    let mut url = Url::parse(&url)?;
    let mut query_pairs: HashMap<String, String> = url
        .query_pairs()
        .map(|(k, v)| (k.into_owned(), v.into_owned()))
        .collect();
    if let Some(paging_value) = query_pairs.get_mut("paging") {
        *paging_value = format!("{};{}", (page - 1) * PAGING_MAX, (page) * PAGING_MAX);
    }
    url.query_pairs_mut().clear();
    for (key, value) in query_pairs.iter() {
        url.query_pairs_mut().append_pair(key, value);
    }
    let response = client.get(url.clone()).send().await?;
    let bytes = response.bytes().await?;
    let channel = Channel::read_from(&bytes[..])?;
    Ok(channel)
}

/// parse channel and return a `(titles, descriptions)` [`tuple`]
pub fn parse_channel(channel: &Channel) -> (Vec<String>, Vec<String>) {
    let items = channel.items().to_vec();
    let re = Regex::new(r"([^\S\n])+").unwrap();
    let titles = items
        .iter()
        .map(|item| {
            let replaced_string = item
                .title()
                .unwrap()
                .to_string()
                .replace("<br />", "\n")
                .replace("\n\n", "\n")
                .replace("<br/>", "\n")
                .replace("<b>", "")
                .replace("</b>", "")
                .replace("<a href=\"", "Link: ")
                .replace("\">click to apply</a>", "")
                .replace("&amp;", "&")
                .replace(" : ", ": ");
            re.replace_all(&decode_html_entities(&replaced_string), " ")
                .to_string()
        })
        .collect::<Vec<String>>();
    let descriptions = items
        .iter()
        .map(|item| {
            let replaced_string = item
                .description()
                .unwrap()
                .to_string()
                .replace("<br />", "\n")
                .replace("\n\n", "\n")
                .replace("<br/>", "\n")
                .replace("<b>", "")
                .replace("</b>", "")
                .replace("<a href=\"", "Link: ")
                .replace("\">click to apply</a>", "")
                .replace("&amp;", "&")
                .replace(" : ", ": ");
            re.replace_all(&decode_html_entities(&replaced_string), " ")
                .to_string()
        })
        .collect::<Vec<String>>();
    (titles, descriptions)
}

/// handle errors raised by the remote connection
pub fn handle_errors(e: Box<dyn Error>) {
    let sources = e.sources();
    let last_source = sources.last().unwrap().to_string();
    let re = Regex::new(r"^.*Temporary failure in name resolution$").unwrap();
    // println!("{}", last_source.to_string());
    if re.is_match(&last_source) {
        error!("No internet connection!");
        error!("Please run without `--get-from-url` or connect to the internet.");
    } else {
        error!("Unknown error.");
    }
}
