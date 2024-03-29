// Standard Uses

// Crate Uses

// External Uses
use axum::{http::StatusCode, routing::post, Json, Router};
use serde::{Deserialize, Serialize};



pub fn register_routes(router: Router) -> Router {
    let router = router
        .route("/api/users/register", post(create_user))
    ;

    router
}

pub async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}

#[derive(Serialize)]
pub struct User {
    id: u64,
    username: String,
}

