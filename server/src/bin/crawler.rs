use parser::{
    config::{get_crawler_config, CrawlerConfig},
    db::{
        article::add_articles,
        feed::{get_feeds_older_than, update_feed_last_updated},
    },
};
use postgres::{Client, NoTls};
use rocket::tokio::runtime::Runtime;

fn main() {
    println!("Initializing crawler");
    let config: CrawlerConfig = get_crawler_config();

    println!("Connecting to DB");
    let mut connection: Client = Client::connect(&config.db_url, NoTls).expect("unable to connect");

    println!("Fetching feeds to refresh");
    let feeds_to_crawl = get_feeds_older_than(&mut connection, &(60 * 60));

    for feed in feeds_to_crawl {
        println!("Crawling... {}", feed.url);
        let rt = Runtime::new().unwrap();
        let articles = rt.block_on(parser::fetch_feed(&feed.url, &feed.id));
        println!("Adding {} new articles", articles.len());
        add_articles(&mut connection, feed.id, articles);
        update_feed_last_updated(&mut connection, &feed.id);
    }

    connection.close().unwrap()
}
