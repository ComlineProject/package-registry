// Standard Uses

// Crate Uses

// External Uses
use axum::Router;
use serde::{Deserialize, Serialize};



pub fn register_routes(router: Router) -> Router {
    router
        //.route("/api/auth/:name/:version", get(find_package_url))
}


#[derive(Serialize)]
pub struct User {
    id: u64,
    username: String,
}

/*
pub async fn oidc_callback(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}
*/
