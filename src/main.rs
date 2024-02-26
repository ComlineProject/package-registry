// Relative Modules
mod registry;
mod storage;
mod web;

// Standard Uses

// Crate Uses

// External Uses


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    //storage::init();

    web::init().await;
}
