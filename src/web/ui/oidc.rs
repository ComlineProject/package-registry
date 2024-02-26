// Standard Uses

// Crate Uses
use crate::storage;
use crate::storage::projects::ProjectInfo;
use crate::web::ui::template::HtmlTemplate;
use super::empty_string_as_none;

// External Uses
use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use askama::Template;
use serde::Deserialize;




pub fn register_routes(router: Router) -> Router {
    let router = router
        .route("/oidc_auth", post(oidc_auth))
        .route("/oidc_callback", post(oidc_callback))
    ;
        
        //.route("/packages", get(|| async { Html(include_str!("../../../templates/packages.html")) }))
        //.route("/package/:name", get(package::package_show))
    router
}


#[derive(Debug, Deserialize)]
struct OIDCAuthParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    foo: Option<i32>,
    bar: Option<String>,
}

#[derive(Template)]
#[template(path = "oidc_auth.html")]
struct OIDCAuthTemplate {}

pub async fn oidc_auth(Query(params): Query<OIDCAuthParams>) -> impl IntoResponse {
    /*
    // get(|| async { Html(include_str!("../../../templates/oidc_callback.html")) })
    let info = storage::projects::find_project(name.clone());
    */
    
    let template = OIDCAuthTemplate {};

    HtmlTemplate(template)
}

pub async fn oidc_callback(Path(name): Path<String>) -> impl IntoResponse {
    let info = storage::projects::find_project(name.clone());

    let template = OIDCAuthTemplate { };

    HtmlTemplate(template)
}
