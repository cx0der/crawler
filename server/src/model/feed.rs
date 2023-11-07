use chrono::{DateTime, Local};
use rocket::serde::{Deserialize, Serialize};
use rss::Channel;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Feed {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub url: String,
    pub icon_url: String,
    pub last_updated: DateTime<Local>,
}

impl Feed {
    pub fn from_channel(url: &str, channel: Channel) -> Feed {
        let icon_url = match channel.image {
            Some(i) => i.url,
            None => "".to_string(),
        };
        Feed {
            id: Uuid::nil(), // ignored.
            name: channel.title,
            description: channel.description,
            url: String::from(url),
            icon_url,
            last_updated: DateTime::UNIX_EPOCH.into(), //ignored
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewFeedRequest {
    pub url: String,
}
