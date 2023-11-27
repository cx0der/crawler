use chrono::{DateTime, Local};
use rocket::serde::{Deserialize, Serialize};
use rss::Item;
use uuid::Uuid;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Article {
    pub id: Option<Uuid>,
    pub feed_id: Uuid,
    pub title: String,
    pub body: String,
    pub url: String,
    pub published_at: DateTime<Local>,
    pub is_read: bool,
    pub is_favourite: bool,
}

impl Article {
    pub fn copy_from_with_id(id: Uuid, other: Article) -> Article {
        Article {
            id: Some(id),
            ..other
        }
    }

    pub fn from_item(item: &Item, feed_id: &Uuid) -> Article {
        Article {
            id: None,
            feed_id: Uuid::from_bytes(feed_id.into_bytes()),
            title: item.title.as_ref().unwrap().to_string(),
            body: item
                .content
                .as_ref()
                .unwrap_or_else(|| item.description.as_ref().unwrap())
                .to_string(),
            url: String::from(item.link.as_ref().unwrap()),
            published_at: DateTime::parse_from_rfc2822(item.pub_date.as_ref().unwrap())
                .unwrap()
                .into(),
            is_read: false,
            is_favourite: false,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct UpdateArticlesState {
    pub ids: Vec<Uuid>,
    pub is_read: Option<bool>,
    pub is_favourite: Option<bool>,
}
