use chrono::TimeDelta;
use libsql::Builder;
use std::{
    env,
    sync::Arc,
};
use ::http::HeaderValue;
use tokio::net::TcpListener;

mod app;
mod cleaner;
mod db;
mod http;
mod surveyor;

use app::{
    App,
    AppConfig,
};

#[tokio::main]
async fn main() {
    let clean_before = env::var("CLEAN_BEFORE").expect("variable CLEAN_BEFORE").parse().expect("parse CLEAN_BEFORE");
    let clean_timeout = env::var("CLEAN_TIMEOUT").expect("variable CLEAN_TIMEOUT").parse().expect("parse CLEAN_BEFORE");
    let database_path = env::var("DATABASE_PATH").expect("variable DATABASE_PATH");
    let host = env::var("HOST").expect("variable HOST");
    let allowed_origins: Vec<HeaderValue> = env::var("ALLOW_ORIGINS").unwrap_or_default()
        .split(" ").map(HeaderValue::from_str).collect::<Result<_, _>>().expect("invalid origin");

    let config = AppConfig {
        clean_before: TimeDelta::seconds(clean_before),
        clean_timeout: TimeDelta::seconds(clean_timeout),
    };

    let database = Builder::new_local(database_path)
        .build()
        .await
        .expect("open database");
    let connection = database.connect().expect("connect");

    let app = App::new(&connection, config).await.expect("create app");
    let router = http::router(Arc::new(app), allowed_origins);

    let listener = TcpListener::bind(host).await.expect("bind");
    axum::serve(listener, router).await.unwrap();
}
