use rustok_core::Result;

#[derive(Debug, Default)]
pub struct OutboxRelay;

impl OutboxRelay {
    pub async fn run(&self) -> Result<()> {
        Ok(())
    }
}
