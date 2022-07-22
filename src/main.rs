mod account;
mod config;
mod database;
mod error;
mod server;

use crate::server::router;
use axum::Router;
use config::{ConfigService, IConfigService};
use database::database_service::{DatabaseService, IDatabaseService};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    // Load in configuration file.
    let config_service = ConfigService {};
    let config = config_service.load_config();

    // Conduct database migrations and initialize database.
    let database_service = DatabaseService {
        config: config.database,
    };
    let db_pool = database_service.init_db();

    // Initialize routes.
    let router: Router = router::init_router();

    // Run the web server.
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
