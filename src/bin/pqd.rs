use pqt::daemon;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    daemon::run().await
}
