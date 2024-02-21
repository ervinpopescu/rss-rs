use std::{collections::HashMap, error::Error};

use html_escape::decode_html_entities;
use regex::Regex;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client, Url,
};
use rss::Channel;
use serde_json::Value;
use url_escape::decode;

const PAGING_MAX: usize = 100;

pub async fn get_channel(url: String, page: usize) -> Result<Channel, Box<dyn Error>> {
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
        *paging_value = format!("{};{}", (page - 1) * PAGING_MAX, page * PAGING_MAX);
    }

    url.query_pairs_mut().clear();
    for (key, value) in query_pairs.iter() {
        url.query_pairs_mut().append_pair(key, value);
    }

    let client = Client::new();
    let json_value: Value = serde_json::from_str(
        r#"{
            "Host": "www.upwork.com",
            "User-Agent": "Mozilla/5.0 (X11; Linux x86_64; rv: 52.0) Gecko/20100101 Thunderbird/52.1.1"
            }
        "#,
    )
    .unwrap();
    let mut headers = HeaderMap::new();
    if let Some(obj) = json_value.as_object() {
        for (key, value) in obj {
            if let Some(header_value) = value.as_str() {
                headers.insert(
                    HeaderName::from_bytes(key.as_bytes()).unwrap(),
                    HeaderValue::from_str(header_value).unwrap(),
                );
            }
        }
    }
    let response = client.get(url.clone()).headers(headers).send().await?;
    let bytes = response.bytes().await?;
    let channel = Channel::read_from(&bytes[..])?;

    Ok(channel)
}

pub fn parse_channel(channel: &Channel) -> (Vec<String>, Vec<String>) {
    let items = channel.items().to_vec();

    let titles = items
        .clone()
        .iter()
        .map(|item| item.title().unwrap().to_string())
        .collect::<Vec<String>>();

    let re = Regex::new(r"([^\S\n])+").unwrap();
    let descriptions = items
        .iter()
        .filter_map(|item| item.description())
        .map(|desc| {
            let replaced_string = desc
                .replace("<br />", "\n")
                .replace("\n\n", "\n")
                .replace("<br/>", "\n")
                .replace("<b>", "")
                .replace("</b>", "")
                .replace("<a href=\"", "Link: ")
                .replace("\">click to apply</a>", "")
                .replace(" : ", ": ");
            re.replace_all(&decode_html_entities(&replaced_string), " ")
                .to_string()
        })
        .collect::<Vec<String>>();
    (titles, descriptions)
}
