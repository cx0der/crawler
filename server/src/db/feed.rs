use std::time::SystemTime;

use chrono::{DateTime, Local};
use postgres::Row;
use rocket_sync_db_pools::postgres::Client;
use uuid::Uuid;

use crate::model::feed::Feed;

pub fn add_feed(connection: &mut Client, feed: &Feed) -> Feed {
    let sql = get_add_feed_query();
    // Set the last updated to Epoch to trigger crawling
    let last_updated: DateTime<Local> = DateTime::from(SystemTime::UNIX_EPOCH);

    let result = connection
        .query_one(
            sql,
            &[
                &feed.name,
                &feed.description,
                &feed.url,
                &feed.icon_url,
                &last_updated,
            ],
        )
        .unwrap();

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
        feeds.push(get_feed_from_row(row));
    }
    feeds
}

pub fn get_feeds_older_than(connection: &mut Client, older_than_secs: &i16) -> Vec<Feed> {
    let sql = format!(
        "SELECT id, name, description, url, icon_url, last_updated FROM feed \
    WHERE last_updated < NOW() - INTERVAL '{} SEC'",
        older_than_secs
    );

    let mut feeds: Vec<Feed> = vec![];

    for row in connection.query(sql.as_str(), &[]).unwrap().iter() {
        feeds.push(get_feed_from_row(row));
    }
    feeds
}

pub fn update_feed_last_updated(connection: &mut Client, feed_id: &Uuid) -> u64 {
    let sql = "UPDATE feed SET last_updated = $1 WHERE id = $2";
    let update_time: DateTime<Local> = Local::now();

    let rows_updated = connection.execute(sql, &[&update_time, &feed_id]).unwrap();
    rows_updated
}

fn get_add_feed_query() -> &'static str {
    "INSERT INTO feed (name, description, url, icon_url, last_updated) \
    VALUES($1, $2, $3, $4, $5) RETURNING id, name, description, url, icon_url, last_updated"
}

fn get_feeds_query() -> &'static str {
    "SELECT id, name, description, url, icon_url, last_updated FROM feed"
}

fn get_feed_from_row(row: &Row) -> Feed {
    let id: Uuid = row.get(0);
    let name: String = row.get(1);
    let description: String = row.get(2);
    let url: String = row.get(3);
    let icon_url: String = row.get(4);
    let last_updated: DateTime<Local> = row.get(5);
    Feed {
        id,
        name,
        description,
        url,
        icon_url,
        last_updated,
    }
}
