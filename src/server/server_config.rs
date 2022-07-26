use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Routes {
    pub health: String,             // Route that checks the server's health.
    pub graphql_api: String,        // Route to the GraphQL API handler.
    pub graphql_playground: String, // Route to the GraphQL playground.
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: IpAddr,   // IP address pointing to the server host address.
    pub port: u16,      // Port that the server application should run on.
    pub routes: Routes, // Collection of server routes.
}
