use parser::config::get_server_app_config;
use parser::db::CrawlyDatabase;
use parser::routes::articles::{get_articles, update_article_read_state};
use parser::routes::feeds::{create_feed, get_all_feeds};
use rocket::{launch, routes};

// Declare all the modules that we plan to use with this main binary

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    let figment = rocket.figment();

    let app_config = get_server_app_config(figment);

    rocket::custom(app_config)
        .mount(
            "/",
            routes![
                get_all_feeds,
                create_feed,
                get_articles,
                update_article_read_state
            ],
        )
        .attach(CrawlyDatabase::fairing())
}
