use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GraphConfig {
    pub max_query_depth: usize, // Maximum query depth allowable by the server (number of nesting levels in query).
    pub max_query_complexity: usize, // Maximum complexity of a query allowable by the server (sum of all field complexities).
}
