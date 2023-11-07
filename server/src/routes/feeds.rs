use reqwest::Url;
use rocket::{get, post, serde::json::Json};

use crate::{
    db::{
        feed::{add_feed, get_feeds},
        CrawlyDatabase,
    },
    fetch_metadata,
    model::feed::{Feed, NewFeedRequest},
};

#[post("/feeds", format = "json", data = "<feed_request>")]
pub async fn create_feed(
    feed_request: Json<NewFeedRequest>,
    db: CrawlyDatabase,
) -> Result<Json<Feed>, String> {
    match Url::parse(feed_request.url.as_str()) {
        Ok(_) => {
            // We have a "valid" looking URL
            let feed = fetch_metadata(&feed_request.url).await;
            let created_feed = db.run(move |c| add_feed(c, &feed)).await;
            Ok(Json(created_feed))
        }
        Err(error) => Err(error.to_string()),
    }
}

#[get("/feeds")]
pub async fn get_all_feeds(db: CrawlyDatabase) -> Result<Json<Vec<Feed>>, String> {
    let result: Vec<Feed> = db.run(|c| get_feeds(c)).await;
    Ok(Json(result))
}
