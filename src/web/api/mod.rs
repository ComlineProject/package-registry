// Relative Modules
mod auth;
mod user;
mod publish;
mod package;

// External Uses
use axum::Router;


pub fn register_routes(router: Router) -> Router {
    let mut router = router;

    router = auth::register_routes(router);
    router = user::register_routes(router);
    router = package::register_routes(router);
    router = publish::register_routes(router);

    router
}
