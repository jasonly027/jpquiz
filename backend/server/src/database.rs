use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::configuration::DatabaseSettings;

pub fn create_pool(config: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(config.connection_options())
}
