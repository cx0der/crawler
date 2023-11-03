use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Channel {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub url: String,
    pub icon_url: String,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ChannelRequest {
    pub url: String,
}
