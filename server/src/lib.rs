use model::{article::Article, feed::Feed};
use rss::Channel;
use uuid::Uuid;

pub mod config;
pub mod db;
pub mod model;

pub fn fetch_metadata(_url: &str) -> Feed {
    unimplemented!()
}

pub fn fetch_feed(url: &str, feed_id: Uuid) -> Vec<Article> {
    let contents = reqwest::blocking::get(url).unwrap().bytes().unwrap();

    let channel = Channel::read_from(&contents[..]).unwrap();

    let mut articles: Vec<Article> = vec![];
    for item in channel.items() {
        articles.push(Article::from_item(item, feed_id));
    }
    articles
}
