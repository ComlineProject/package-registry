// Relative Modules
pub mod api;
pub mod ui;

// Standard Uses
use std::net::SocketAddr;

// Crate Uses

// External Uses
use axum::Router;
use once_cell::sync::OnceCell;


pub static ADDRESS: OnceCell<String> = OnceCell::new();
pub fn web_address() -> String { format!("http://{}", ADDRESS.get().unwrap()) }

pub async fn init() {
    let (address, port) = ([127, 0, 0 ,1], 3000);
    
    let app = Router::new();
    let app = api::register_routes(app);
    let app = ui::register_routes(app);
    
    let addr = SocketAddr::from((address, port));
    ADDRESS.get_or_init(|| addr.to_string());

    tracing::info!("Listening on http://{}/", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
