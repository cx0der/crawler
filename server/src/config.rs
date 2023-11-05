use std::env;

const DB_URL: &str = "APP_DATABASE_URL";
const REFRESH_INTERNAL: &str = "INTERVAL";

pub fn get_server_app_config(base_config: &rocket::figment::Figment) -> rocket::figment::Figment {
    use rocket::figment::{
        providers::Env,
        util::map,
        value::{Map, Value},
        Figment,
    };

    // use dotenv to load up env
    load_env_file();

    // rocket_sync_pool expects a "map" for DB config.
    // only mandatory field is url
    let db_config: Map<_, Value> = map! {
        "url" => dotenv::var(DB_URL).unwrap().into(),
        "timeout" => 5.into(),
        "pool_size" => 5.into(),
    };

    let updated_figment = Figment::new()
        .merge(base_config)
        .merge(Env::prefixed("APP_"))
        .merge(("databases", map!["crawly_db" => db_config]));

    updated_figment
}

pub struct CrawlerConfig {
    pub db_url: String,
    pub refresh_interval: u16,
}

pub fn get_crawler_config() -> CrawlerConfig {
    load_env_file();

    CrawlerConfig {
        db_url: dotenv::var(DB_URL).unwrap(),
        refresh_interval: dotenv::var(REFRESH_INTERNAL)
            .unwrap_or_else(|_| "3600".to_string())
            .parse::<u16>()
            .unwrap(),
    }
}

fn load_env_file() {
    // use dotenv to load up env
    let env_path = env::current_dir().and_then(|a| Ok(a.join(".env"))).unwrap();
    let _ = dotenv::from_path(env_path.as_path());
}
