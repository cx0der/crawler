use std::env;

use rocket::figment::{
    providers::Env,
    util::map,
    value::{Map, Value},
    Figment,
};

pub fn get_app_config(base_config: &Figment) -> Figment {
    // use dotenv to load up env
    let env_path = env::current_dir().and_then(|a| Ok(a.join(".env"))).unwrap();
    let _ = dotenv::from_path(env_path.as_path());

    // rocket_sync_pool expects a "map" for DB config.
    // only mandatory field is url
    let db_config: Map<_, Value> = map! {
        "url" => dotenv::var("APP_DATABASE_URL").unwrap().into(),
        "timeout" => 5.into(),
        "pool_size" => 5.into(),
    };

    let updated_figment = Figment::new()
        .merge(base_config)
        .merge(Env::prefixed("APP_"))
        .merge(("databases", map!["crawly_db" => db_config]));

    updated_figment
}
