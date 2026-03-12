//! Canonical event contracts crate for RusToK.

mod schema;
mod types;
pub mod validation;

pub use schema::{event_schema, EventSchema, FieldSchema, EVENT_SCHEMAS};
pub use types::{DomainEvent, EventEnvelope};
pub use validation::{EventValidationError, ValidateEvent};

pub use DomainEvent as RootDomainEvent;
pub use EventEnvelope as RootEventEnvelope;

#[cfg(test)]
mod contract_tests;
