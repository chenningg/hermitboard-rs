use crate::schema::{self, UnifiedSchema};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{self, IntoResponse},
    routing::{get, post},
    Extension, Router,
};

// Handles GraphQL requests.
async fn graphql_handler(schema: Extension<UnifiedSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

// Generates the GraphQL playground.
async fn graphql_playground() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

// Initializes routing routes.
pub fn init_router() -> Router {
    let schema = schema::build_schema().finish();

    let router = Router::new()
        // Health check route.
        .route("/health", get(|| async { "Healthy! ^^" }))
        // GraphQL api route.
        .route("/api", post(graphql_handler))
        // GraphQL playground route.
        .route("/graphql", get(graphql_playground))
        .layer(Extension(schema));

    router
}
