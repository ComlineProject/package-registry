// Relative Modules
mod oidc;
mod template;

// Standard Uses
use std::fmt;
use std::str::FromStr;

// Crate Uses

// External Uses
use axum::Router;
use axum::routing::get;
use axum::response::Html;
use serde::{de, Deserialize, Deserializer};


pub fn register_routes(router: Router) -> Router {
    let router = router
        .route("/", get(|| async { Html(include_str!("../../../templates/index.html")) }))
    ;        
    
    oidc::register_routes(router)
}

pub fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}
