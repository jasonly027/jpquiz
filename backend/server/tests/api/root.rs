use tracing::info;

use crate::helpers::test_router;

#[tokio::test]
async fn root_works() {
    let _ = test_router().await;
    info!("Logging in test");
}
