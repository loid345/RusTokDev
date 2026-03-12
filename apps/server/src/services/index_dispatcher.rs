use loco_rs::app::AppContext;
use rustok_core::events::EventDispatcher;
use rustok_index::content::ContentIndexer;
use rustok_index::product::ProductIndexer;
use rustok_index::IndexerRuntimeConfig;
use rustok_telemetry::metrics;

use crate::common::settings::RustokSettings;

pub fn spawn_index_dispatcher(ctx: &AppContext, rustok_settings: &RustokSettings) {
    let bus = crate::services::event_bus::event_bus_from_context(ctx);
    let db = ctx.db.clone();
    let runtime = IndexerRuntimeConfig::new(
        rustok_settings.search.reindex.parallelism,
        rustok_settings.search.reindex.entity_budget,
        rustok_settings.search.reindex.yield_every,
    );
    metrics::record_index_reindex_runtime_config(
        "content_indexer",
        rustok_settings.search.reindex.parallelism,
        rustok_settings.search.reindex.entity_budget,
        rustok_settings.search.reindex.yield_every,
    );
    metrics::record_index_reindex_runtime_config(
        "product_indexer",
        rustok_settings.search.reindex.parallelism,
        rustok_settings.search.reindex.entity_budget,
        rustok_settings.search.reindex.yield_every,
    );
    tracing::info!(
        reindex_parallelism = rustok_settings.search.reindex.parallelism,
        reindex_entity_budget = rustok_settings.search.reindex.entity_budget,
        reindex_yield_every = rustok_settings.search.reindex.yield_every,
        "Initialized index dispatcher runtime contract"
    );

    let mut dispatcher = EventDispatcher::new(bus);
    dispatcher.register(ContentIndexer::with_runtime(db.clone(), runtime.clone()));
    dispatcher.register(ProductIndexer::with_runtime(db, runtime));

    let running = dispatcher.start();

    tokio::spawn(async move {
        if let Err(error) = running.join().await {
            tracing::error!("Index dispatcher task panicked: {:?}", error);
        }
    });
}
