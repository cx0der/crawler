use rocket::{get, put, serde::json::Json};

use crate::{
    db::{
        article::{get_all_unread_articles, set_articles_read_state},
        CrawlyDatabase,
    },
    model::article::{Article, UpdateArticlesState},
};

#[get("/articles")]
pub async fn get_articles(db: CrawlyDatabase) -> Result<Json<Vec<Article>>, String> {
    let articles = db.run(|c| get_all_unread_articles(c)).await;

    Ok(Json(articles))
}

#[put("/articles", format = "json", data = "<req>")]
pub async fn update_article_read_state(
    req: Json<UpdateArticlesState>,
    db: CrawlyDatabase,
) -> Result<String, String> {
    if req.is_read.is_none() && req.is_favourite.is_none() {
        return Err(String::from(
            "Atleast one of isRead or isFavourite should be set",
        ));
    }
    db.run(move |c| set_articles_read_state(c, req.is_read, req.is_favourite, &req.ids))
        .await;
    Ok("OK".to_string())
}
