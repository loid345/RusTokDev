//! RusToK Index - CQRS Read Model
//!
//! Denormalized indexes for fast reads and full-text search.

pub mod content;
pub mod error;
pub mod product;
pub mod search;
pub mod traits;

pub use error::{IndexError, IndexResult};
pub use traits::{Indexer, IndexerContext, LocaleIndexer};
