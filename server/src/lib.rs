use model::{article::Article, feed::Feed};
use rss::Channel;
use uuid::Uuid;

pub mod config;
pub mod db;
pub mod model;
pub mod routes;

pub async fn fetch_metadata(url: &str) -> Feed {
    let channel = fetch_channel(url).await;
    Feed::from_channel(url, channel)
}

pub async fn fetch_feed(url: &str, feed_id: &Uuid) -> Vec<Article> {
    let channel = fetch_channel(url).await;

    let mut articles: Vec<Article> = vec![];
    for item in channel.items() {
        articles.push(Article::from_item(item, feed_id));
    }
    articles
}

async fn fetch_channel(url: &str) -> Channel {
    let contents = reqwest::get(url).await.unwrap().bytes().await.unwrap();
    Channel::read_from(&contents[..]).unwrap()
}
