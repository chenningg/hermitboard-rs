use crate::account::account_query::AccountQuery;
use crate::server::health_query::HealthQuery;

use async_graphql::MergedObject;

// Combine all queries into a root query.
#[derive(MergedObject, Default)]
pub struct Query(HealthQuery, AccountQuery);
