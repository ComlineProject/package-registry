// Standard Uses

// Crate Uses
use crate::storage::FROZEN_PROJECT_RELATIVE_DIR;
use crate::web;

// External Uses
use axum::{extract::Path, http::StatusCode, Router};
use axum::routing::get;
use serde::Deserialize;


#[derive(Deserialize)]
pub struct PackageMeta {
    name: String,
    version: String
}


pub fn register_routes(router: Router) -> Router {
    router
        .route("/api/package/:name/:version", get(find_package_url))
}


/*
pub async fn find_package_url(Json(payload): Json<PackageMeta>) -> (StatusCode, Json<String>) {
    let package_url = "".to_owned();

    (StatusCode::CREATED, Json(package_url))
}
*/

pub async fn find_package_url(
    Path((name, version)): Path<(String, String)>
) -> (StatusCode, String) {
    let url = format!(
        "{}/{}/{}{}",
        web::web_address(), FROZEN_PROJECT_RELATIVE_DIR, name, version
    );

    (StatusCode::OK, url)
}

