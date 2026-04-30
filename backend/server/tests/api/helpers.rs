use std::sync::{Arc, LazyLock};

use axum::Router;
use server::{
    application::{self, AppStateInternal},
    configuration::{self, DatabaseSettings},
    telemetry,
};
use sqlx::{Connection, Executor, PgConnection, PgPool, migrate};
use tracing::debug;
use uuid::Uuid;

pub static INIT: LazyLock<()> = LazyLock::new(|| {
    dotenvy::dotenv().ok();
    telemetry::init("TEST_LOG");
});

pub async fn test_router() -> Router {
    LazyLock::force(&INIT);

    let config = {
        let mut cfg = configuration::get().expect("Failed to read configuration");
        cfg.database.database = Uuid::now_v7().to_string(); // Each test gets their own database
        cfg
    };

    let db_pool = test_pool(&config.database).await;
    let state = Arc::new(AppStateInternal { db_pool });

    let (router, _) = application::router().split_for_parts();

    router.with_state(state)
}

#[tracing::instrument(level = "debug", name = "Creating test database pool", fields(database_name = config.database), skip(config))]
async fn test_pool(config: &DatabaseSettings) -> PgPool {
    // Create a fresh database every time
    {
        debug!("Creating database");

        let maintence_config = DatabaseSettings {
            database: "postgres".to_string(),
            ..config.clone()
        };

        let mut connection = PgConnection::connect_with(&maintence_config.connection_options())
            .await
            .expect("Failed to connect to database");

        connection
            .execute(format!(r#"CREATE DATABASE "{}";"#, config.database).as_str())
            .await
            .expect("Failed to create test database");
    }

    let pool = PgPool::connect_with(config.connection_options())
        .await
        .expect("Failed to connect to database");

    debug!("Applying migrations");
    migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate database");

    pool
}
