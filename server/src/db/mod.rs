// pub const DATE_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";

use rocket_sync_db_pools::{database, postgres};

pub mod article;
pub mod feed;

#[database("crawly_db")]
pub struct CrawlyDatabase(postgres::Client);
