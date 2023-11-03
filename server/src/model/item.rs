use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct Item {
    pub item_id: Uuid,
    pub channel_id: Uuid,
    pub item_title: String,
    pub item_description: String,
    pub item_url: String,
    pub item_timestamp: DateTime<Utc>,
    pub is_read: bool,
}
