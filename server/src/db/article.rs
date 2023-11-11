use postgres::{Client, Row};
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

pub fn get_all_unread_articles(connection: &mut Client) -> Vec<Article> {
    let sql = get_all_unread_articles_query();

    let mut unread_articles: Vec<Article> = vec![];

    for row in connection.query(sql, &[]).unwrap() {
        unread_articles.push(get_article_from_row(&row));
    }

    unread_articles
}

pub fn _get_all_articles_for_feed(connection: &mut Client, feed_id: &Uuid) -> Vec<Article> {
    let sql = "SELECT id, feed_id, title, body, url, published_at, is_read \
    FROM article WHERE is_read = FALSE AND feed_id = $1 GROUP BY feed_id";

    let mut articles: Vec<Article> = vec![];

    for row in connection.query(sql, &[feed_id]).unwrap() {
        articles.push(get_article_from_row(&row));
    }
    articles
}

pub fn set_articles_read_state(connection: &mut Client, read_state: bool, ids: &Vec<Uuid>) -> u64 {
    let in_clause = ids
        .iter()
        .map(|x| format!("'{}'", x.as_hyphenated().to_string()))
        .collect::<Vec<_>>()
        .join(",");
    let sql = format!(
        "UPDATE article SET is_read = $1 WHERE id IN ({})",
        &in_clause
    );
    println!("{}", sql);
    let count = connection.execute(&sql, &[&read_state]).unwrap();
    count
}

fn get_add_article_query() -> &'static str {
    "INSERT INTO article (feed_id, title, body, url, published_at, is_read) \
    VALUES ($1, $2, $3, $4, $5, $6) RETURNING id"
}

fn get_all_unread_articles_query() -> &'static str {
    "SELECT id, feed_id, title, body, url, published_at, is_read FROM article \
    WHERE is_read = FALSE"
}

fn get_article_from_row(row: &Row) -> Article {
    Article {
        id: row.get(0),
        feed_id: row.get(1),
        title: row.get(2),
        body: row.get(3),
        url: row.get(4),
        published_at: row.get(5),
        is_read: row.get(6),
    }
}
