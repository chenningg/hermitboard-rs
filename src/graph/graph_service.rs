use super::graph_config::GraphConfig;
use super::schema::{UnifiedSchema, UnifiedSchemaBuilder};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use async_trait::async_trait;
use axum::{extract::Extension, response::Html};

#[async_trait]
pub trait GraphService {
    async fn new(graph_config: &GraphConfig, schema_builder: UnifiedSchemaBuilder) -> Self;

    async fn graphql_handler(
        schema: Extension<UnifiedSchema>,
        req: GraphQLRequest,
    ) -> GraphQLResponse {
        schema.execute(req.into_inner()).await.into()
    }

    async fn graphql_playground(api_route: Extension<&str>) -> Html<String> {
        Html(playground_source(GraphQLPlaygroundConfig::new(&api_route)))
    }

    fn schema(&self) -> &UnifiedSchema;
}

// Implementation of the database service.
pub struct GraphServiceImpl {
    schema: UnifiedSchema,
}

#[async_trait]
impl GraphService for GraphServiceImpl {
    // Creates a new database service with a database pool of connections.
    async fn new(graph_config: &GraphConfig, schema_builder: UnifiedSchemaBuilder) -> Self {
        // Build the completed schema with graph settings.
        let schema = schema_builder
            .limit_depth(graph_config.max_query_depth)
            .limit_complexity(graph_config.max_query_complexity)
            .finish();

        GraphServiceImpl { schema }
    }

    fn schema(&self) -> &UnifiedSchema {
        &self.schema
    }
}
