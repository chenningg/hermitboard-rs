pub mod database_config;

use sqlx::postgres::PgPoolOptions;

pub fn init_db() {
    // let pool = PgPoolOptions::new()
    //     .max_connections(5)
    //     .connect("postgres://postgres:password@localhost/test")
    //     .await?;
}
