use server::{application::Application, configuration, telemetry};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    telemetry::init("RUST_LOG");

    let config = configuration::get().expect("Failed to read configuration");

    let app = Application::build(&config).await?;

    Ok(app.run().await?)
}
