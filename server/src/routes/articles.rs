use rocket::{get, put, serde::json::Json};

use crate::{
    db::{
        article::{get_all_unread_articles, set_articles_read_state},
        CrawlyDatabase,
    },
    model::article::{Article, UpdateArticles},
};

#[get("/articles")]
pub async fn get_articles(db: CrawlyDatabase) -> Result<Json<Vec<Article>>, String> {
    let articles = db.run(|c| get_all_unread_articles(c)).await;

    Ok(Json(articles))
}

#[put("/articles", format = "json", data = "<req>")]
pub async fn update_article_read_state(
    req: Json<UpdateArticles>,
    db: CrawlyDatabase,
) -> Result<String, String> {
    db.run(move |c| set_articles_read_state(c, req.is_read, &req.ids))
        .await;
    Ok("OK".to_string())
}
