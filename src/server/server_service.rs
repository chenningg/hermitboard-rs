use super::server_config::ServerConfig;
use crate::graph::graph_service::GraphService;
use async_trait::async_trait;
use axum::{
    headers::Server,
    routing::{get, post},
    Extension, Router,
};
use std::net::IpAddr;

#[async_trait]
pub trait ServerService {
    async fn new<T>(server_config: &'static ServerConfig, graph_service: &'static T) -> Self
    where
        T: 'static + GraphService + Sync;

    fn router(&self) -> &Router;
    fn config(&self) -> &ServerConfig;
}

// Implementation of the database service.
pub struct ServerServiceImpl {
    router: Router,       // The router for the server.
    config: ServerConfig, // The config for the server.
}

#[async_trait]
impl ServerService for ServerServiceImpl {
    async fn new<T>(server_config: &'static ServerConfig, graph_service: &'static T) -> Self
    where
        T: 'static + GraphService + Sync,
    {
        // Create a new router with routes from server config.
        let router = Router::new()
            // Health check route.
            .route(
                &server_config.routes.health,
                get(|| async { "Healthy! ^^" }),
            )
            // GraphQL api route.
            .route(&server_config.routes.graphql_api, post(T::graphql_handler))
            // GraphQL playground route.
            .route(
                &server_config.routes.graphql_playground,
                get(T::graphql_playground),
            )
            .layer(Extension(graph_service.schema()))
            .layer(Extension(&server_config.routes.graphql_playground));

        ServerServiceImpl {
            router,
            config: server_config,
        }
    }

    fn router(&self) -> &Router {
        &self.router
    }

    fn config(&self) -> &ServerConfig {
        &self.config
    }
}
