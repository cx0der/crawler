use chrono::{DateTime, Utc};
use rocket_sync_db_pools::postgres::Client;
use uuid::Uuid;

use crate::model::channel::Channel;

pub fn add_channel(connection: &mut Client, url: &str) -> Channel {
    let id: Uuid = Uuid::new_v4();
    let last_updated: DateTime<Utc> = Utc::now();
    let sql = "INSERT INTO channels (channel_id, channel_name, channel_description, \
        channel_url, channel_icon_url, last_updated) VALUES($1, '', '', $2, '', $3)";

    connection
        .execute(sql, &[&id, &url, &last_updated])
        .unwrap();

    Channel {
        id,
        name: "".to_string(),
        description: "".to_string(),
        url: url.to_string(),
        icon_url: "".to_string(),
        last_updated,
    }
}

pub fn get_all_channels(connection: &mut Client) -> Vec<Channel> {
    let sql = "SELECT channel_id, channel_name, channel_description, channel_url,\
    channel_icon_url, last_updated FROM channels";

    let mut results: Vec<Channel> = vec![];

    for row in connection.query(sql, &[]).unwrap().iter() {
        let id: Uuid = row.get(0);
        let name: String = row.get(1);
        let description: String = row.get(2);
        let url: String = row.get(3);
        let icon_url: String = row.get(4);
        let last_updated: DateTime<Utc> = row.get(5);

        let ch = Channel {
            id,
            name,
            description,
            url,
            icon_url,
            last_updated,
        };
        results.push(ch);
    }

    results
}
