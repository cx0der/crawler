use crate::{
    db::{
        channel::{add_channel, get_all_channels},
        CrawlyDatabase,
    },
    model::channel::{Channel, ChannelRequest},
};
use rocket::{get, post, serde::json::Json};

#[get("/channels")]
pub async fn get_channels(db: CrawlyDatabase) -> Result<Json<Vec<Channel>>, String> {
    let result: Vec<Channel> = db.run(|c| get_all_channels(c)).await;
    Ok(Json(result))
}

#[post("/channels", format = "json", data = "<new_channel>")]
pub async fn create_channel(
    new_channel: Json<ChannelRequest>,
    db: CrawlyDatabase,
) -> Result<Json<Channel>, String> {
    let created_channel = db.run(move |c| add_channel(c, &new_channel.url)).await;
    Ok(Json(created_channel))
}
