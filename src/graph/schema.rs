use super::query::Query;
use async_graphql::{EmptyMutation, EmptySubscription, Schema, SchemaBuilder};

// Unified GraphQL schema builder type
pub type UnifiedSchemaBuilder = SchemaBuilder<Query, EmptyMutation, EmptySubscription>;

// Unified GraphQL schema type
pub type UnifiedSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn build_schema() -> UnifiedSchemaBuilder {
    Schema::build(Query::default(), EmptyMutation, EmptySubscription)
}
