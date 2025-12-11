use axum::{
    extract::{
        Json,
        State,
    },
    http::{
        StatusCode,
    },
    routing::get,
    Router,
};
use chrono::{
    Local,
};
use std::{
    sync::{
        Arc,
    },
};

use crate::{
    app::App,
    surveyor::Summary,
};

async fn submit(
    State(app): State<Arc<App<Local>>>,
    Json(answer): Json<bool>
) -> Result<(), StatusCode> {
    app.submit(Local::now(), answer)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn summary(
    State(app): State<Arc<App<Local>>>,
) -> Result<Json<Summary<i64>>, StatusCode> {
    let summary = app
        .summary(Local::now())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(summary))
}

pub fn router(app: Arc<App<Local>>) -> Router {
    Router::new()
        .route("/", get(summary).post(submit))
        .with_state(app)
}
