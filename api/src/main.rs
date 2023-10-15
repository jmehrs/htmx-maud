use std::sync::{Arc, Mutex};

use anyhow::Result;
use api::{device::device_router, index, init_db};
use axum::{routing::get, Router};
use tower_http::services::ServeDir; // TODO: Replace this with ThisError errors

#[tokio::main]
async fn main() -> Result<()> {
    let conn = init_db("./app.db")?;
    let conn = Arc::new(Mutex::new(conn));

    // build our application with a single route
    let app = Router::new()
        .route("/", get(index))
        .nest("/device", device_router())
        .with_state(conn)
        .nest_service("/assets", ServeDir::new("assets"));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
