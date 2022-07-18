mod schema;
mod server;

use axum::Router;
use server::router;

#[tokio::main]
async fn main() {
    // Initialize routes.
    let router: Router = router::init_router();

    // Run the web server.
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
