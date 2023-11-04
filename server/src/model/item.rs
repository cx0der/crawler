use chrono::{DateTime, Local};
use uuid::Uuid;

#[derive(Debug)]
pub struct Item {
    pub id: Uuid,
    pub feed_id: Uuid,
    pub title: String,
    pub description: String,
    pub url: String,
    pub published_at: DateTime<Local>,
    pub is_read: bool,
}
