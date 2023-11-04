use config::get_app_config;
use db::CrawlyDatabase;
use rocket::{launch, routes};
use routes::feeds::{create_feed, get_all_feeds};

// Declare all the modules that we plan to use with this main binary
mod config;
mod db;
mod model;
mod routes;

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    let figment = rocket.figment();

    let app_config = get_app_config(figment);

    rocket::custom(app_config)
        .mount("/", routes![get_all_feeds, create_feed])
        .attach(CrawlyDatabase::fairing())
}
