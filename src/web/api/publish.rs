// Standard Uses

// Crate Uses

// External Uses
use axum::{http::StatusCode, Json, Router, routing::post};
use serde::Deserialize;


pub fn register_routes(router: Router) -> Router {
    let router = router
        .route("/api/publish/package", post(publish_package))
    ;

    router
}

#[allow(unused)]
pub async fn publish_package(Json(payload): Json<PackageDetails>) -> (StatusCode, Json<String>) {
    let package_url = "".to_owned();

    (StatusCode::CREATED, Json(package_url))
}


#[allow(unused)]
#[derive(Deserialize)]
pub struct PackageDetails {
    repository_url: String
}

