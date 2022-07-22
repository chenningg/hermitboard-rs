mod account;
mod config;
mod database;
mod server;

use crate::server::router;
use axum::Router;
use config::{ConfigService, IConfigService};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    // Load in configuration file.
    let config_service = ConfigService {};
    let config = config_service.load_config();

    // Construct a subscriber that prints formatted traces to stdout.
    let subscriber = FmtSubscriber::new();

    // Initialize routes.
    let router: Router = router::init_router();

    // Run the web server.
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
