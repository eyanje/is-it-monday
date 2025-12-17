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
use http::{
    header::{CONTENT_TYPE},
    Method,
};
use std::{
    sync::{
        Arc,
    },
};
use tower_http::cors::{
    AllowOrigin,
    CorsLayer,
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

pub fn router<O>(app: Arc<App<Local>>, allowed_origins: O) -> Router
where O: Into<AllowOrigin> {
    let cors_layer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE])
        .allow_origin(allowed_origins);

    Router::new()
        .route("/", get(summary).post(submit))
        .layer(cors_layer)
        .with_state(app)
}
