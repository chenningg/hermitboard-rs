mod account;
mod config;
mod database;
mod error;
mod graph;
mod server;
mod traits;

use config::config_service::{ConfigService, ConfigServiceImpl};
use database::database_service::{DatabaseService, DatabaseServiceImpl};
use dotenv;
use graph::{
    graph_service::{GraphService, GraphServiceImpl},
    schema,
};
use server::server_service::{ServerService, ServerServiceImpl};
use tracing::error;

#[tokio::main]
async fn main() {
    // Load in environment variables
    dotenv::dotenv().ok();

    // Load in configuration file.
    let config_service = ConfigServiceImpl::new().load_config();
    let config = config_service.config();

    // Conduct database migrations and initialize database.
    let database_service = DatabaseServiceImpl::new(&config.database);

    // Initialize graphql schema builder.
    let schema_builder = schema::build_schema();

    // Create graph service.
    let graph_service = GraphServiceImpl::new(&config.graph, schema_builder);

    // Create the server and router.
    let server_service = ServerServiceImpl::new(&config.server, &graph_service);

    // Run the web server.
    axum::Server::bind(
        &format!(
            "{}:{}",
            server_service.config().host,
            server_service.config().to_string()
        )
        .parse()
        .unwrap_or_else(|err| {
            // There was an error parsing the server address, panic.
            error!(
                "{} for provided address: {}",
                err,
                format!(
                    "{}:{}",
                    server_service.config().host,
                    server_service.config().to_string()
                )
            );
            panic!(
                "{} for provided address: {}",
                err,
                format!(
                    "{}:{}",
                    server_service.config().host,
                    server_service.config().to_string()
                )
            );
        }),
    )
    .serve(server_service.router().into_make_service())
    .await
    .unwrap();
}
