// Standard Uses

// Crate Uses

// External Uses
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};



pub async fn oidc_callback(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
pub struct  {
    username: String,
}

#[derive(Serialize)]
pub struct User {
    id: u64,
    username: String,
}

