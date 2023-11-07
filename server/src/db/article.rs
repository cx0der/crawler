use std::collections::HashMap;

use postgres::Client;
use uuid::Uuid;

use crate::model::article::Article;

pub fn add_articles(
    connection: &mut Client,
    feed_id: Uuid,
    articles: Vec<Article>,
) -> Vec<Article> {
    let sql = get_add_article_query();
    let stmt = connection.prepare(sql).unwrap();

    let mut added_items: Vec<Article> = vec![];

    for article in articles {
        let row = connection
            .query_one(
                &stmt,
                &[
                    &feed_id,
                    &article.title,
                    &article.body,
                    &article.url,
                    &article.published_at,
                    &article.is_read,
                ],
            )
            .unwrap();
        let id = row.get(0);
        added_items.push(Article::copy_from_with_id(id, article));
    }

    added_items
}

pub fn mark_article_as_read(connection: &mut Client, item_id: &Uuid) -> bool {
    set_article_read_state(connection, true, item_id)
}

pub fn _mark_article_as_unread(connection: &mut Client, item_id: &Uuid) -> bool {
    set_article_read_state(connection, false, item_id)
}

pub fn _get_all_unread_articles(_connection: &mut Client) -> HashMap<Uuid, Vec<Article>> {
    unimplemented!()
}

pub fn _get_all_articles_for_feed(_connection: &mut Client, _feed_id: &Uuid) -> Vec<Article> {
    unimplemented!()
}

fn set_article_read_state(connection: &mut Client, read_state: bool, item_id: &Uuid) -> bool {
    let sql = get_article_read_state_update_query();

    connection.execute(sql, &[&read_state, &item_id]).unwrap();

    read_state
}

fn get_article_read_state_update_query() -> &'static str {
    "UPDATE article SET is_read = $1 WHERE id = $2"
}

fn get_add_article_query() -> &'static str {
    "INSERT INTO article (feed_id, title, body, url, published_at, is_read) \
    VALUES ($1, $2, $3, $4, $5, $6) RETURNING id"
}
