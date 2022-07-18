use async_graphql::Object;

#[derive(Default)]
pub struct HealthQuery;

#[Object]
impl HealthQuery {
    // Returns true to signify that the server is reachable.
    async fn health(&self) -> bool {
        true
    }
}