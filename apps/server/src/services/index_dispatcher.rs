use loco_rs::app::AppContext;
use rustok_core::events::EventDispatcher;
use rustok_index::content::ContentIndexer;
use rustok_index::product::ProductIndexer;

pub fn spawn_index_dispatcher(ctx: &AppContext) {
    let bus = crate::services::event_bus::event_bus_from_context(ctx);
    let db = ctx.db.clone();

    let mut dispatcher = EventDispatcher::new(bus);
    dispatcher.register(ContentIndexer::new(db.clone()));
    dispatcher.register(ProductIndexer::new(db));

    let running = dispatcher.start();

    tokio::spawn(async move {
        if let Err(error) = running.join().await {
            tracing::error!("Index dispatcher task panicked: {:?}", error);
        }
    });
}
