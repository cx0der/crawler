use chrono::{DateTime, Local};
use rocket_sync_db_pools::postgres::Client;
use uuid::Uuid;

use crate::model::feed::Feed;

pub fn add_feed(connection: &mut Client, url: &str) -> Feed {
    let sql = get_add_feed_sql();

    let result = connection.query_one(sql, &[&url]).unwrap();

    Feed {
        id: result.get(0),
        name: result.get(1),
        description: result.get(2),
        url: result.get(3),
        icon_url: result.get(4),
        last_updated: result.get(5),
    }
}

pub fn get_feeds(connection: &mut Client) -> Vec<Feed> {
    let sql = get_feeds_query();
    let prepared_stmt = connection.prepare(sql).unwrap();

    let mut feeds: Vec<Feed> = vec![];
    for row in connection.query(&prepared_stmt, &[]).unwrap().iter() {
        let id: Uuid = row.get(0);
        let name: String = row.get(1);
        let description: String = row.get(2);
        let url: String = row.get(3);
        let icon_url: String = row.get(4);
        let last_updated: DateTime<Local> = row.get(5);

        feeds.push(Feed {
            id,
            name,
            description,
            url,
            icon_url,
            last_updated,
        });
    }
    feeds
}

fn get_add_feed_sql() -> &'static str {
    "INSERT INTO feed (name, description, url, icon_url) \
  VALUES('', '', $1, '') RETURNING id, name, description, url, icon_url, last_updated"
}

fn get_feeds_query() -> &'static str {
    "SELECT id, name, description, url, icon_url, last_updated FROM feed"
}
