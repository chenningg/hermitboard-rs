use crate::server::health_query::HealthQuery;

use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema, SchemaBuilder};

// Combine all queries into a root query.
#[derive(MergedObject, Default)]
pub struct Query(HealthQuery);

// Unified GraphQL schema type
pub type UnifiedSchema = Schema<Query, EmptyMutation, EmptySubscription>;

// Builds the unified GraphQL schema.
pub fn build_schema() -> SchemaBuilder<Query, EmptyMutation, EmptySubscription> {
    Schema::build(Query::default(), EmptyMutation, EmptySubscription)
}
