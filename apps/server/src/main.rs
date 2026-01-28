use loco_rs::{cli, Result};
use migration::Migrator;
use rustok_server::app::App;

#[tokio::main]
async fn main() -> Result<()> {
    cli::main::<App, Migrator>().await
}
