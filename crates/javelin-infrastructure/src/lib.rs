// Infrastructure Layer - 永続化 / 外部技術実装
// 依存方向: → Domain
// 現代Rust設計: LMDB + CQRS + Event Sourcing 最適化

pub mod commands;
pub mod error;
pub mod event_handlers;
pub mod journal_entry_finder_impl;
pub mod ledger_query_service_impl;
pub mod queries;
pub mod repositories;
pub mod services;
pub mod storage_metrics;
pub mod types;

// Event Store modules
#[path = "event_store/event_store.rs"]
pub mod event_store;
#[path = "event_store/event_store_repository_impl.rs"]
pub mod event_store_repository_impl;
#[path = "event_store/event_stream.rs"]
pub mod event_stream;
#[path = "event_store/snapshot_db.rs"]
pub mod snapshot_db;

// Projection modules
#[path = "projections/projection_builder_impl.rs"]
pub mod projection_builder_impl;
#[path = "projections/projection_db.rs"]
pub mod projection_db;
#[path = "projections/projection_trait.rs"]
pub mod projection_trait;
#[path = "projections/projection_worker.rs"]
pub mod projection_worker;

// Test modules
#[cfg(test)]
#[path = "tests/event_store_property_tests.rs"]
mod event_store_property_tests;
#[cfg(test)]
#[path = "tests/event_store_unit_tests.rs"]
mod event_store_unit_tests;
#[cfg(test)]
#[path = "tests/ledger_query_service_property_tests.rs"]
mod ledger_query_service_property_tests;
#[cfg(test)]
#[path = "tests/projection_builder_property_tests.rs"]
mod projection_builder_property_tests;

// Re-export for convenience
pub use commands::{
    AccountingPeriodRepositoryImpl, JournalEntryRepositoryImpl, UserActionRepositoryImpl,
};
pub use event_handlers::journal_entry_event_handler;
pub use event_store::EventStore;
pub use event_stream::{EventStream, EventStreamBuilder, EventStreamIterator, StoredEvent};
pub use journal_entry_finder_impl::JournalEntryFinderImpl;
pub use ledger_query_service_impl::LedgerQueryServiceImpl;
pub use projection_builder_impl::ProjectionBuilderImpl;
pub use projection_db::{ProjectionDb, ProjectionPosition};
pub use projection_trait::{Apply, ProjectEvent, ProjectionStrategy, ToReadModel};
pub use projection_worker::ProjectionWorker;
pub use queries::{
    journal_entry_projection, journal_entry_projection_worker, ledger_projection,
    master_data_loader_impl,
};
pub use repositories::{
    AccountMasterRepositoryImpl, ApplicationSettingsRepositoryImpl, CompanyMasterRepositoryImpl,
};
pub use services::VoucherNumberGeneratorImpl;
pub use snapshot_db::{
    EveryNEvents, EveryNMinutes, Snapshot, SnapshotDb, SnapshotEvery60Min, SnapshotEvery100,
    SnapshotEvery1000, SnapshotPolicyTrait,
};
pub use storage_metrics::{DurabilityPolicy, ProjectionLagMetrics, StorageMetrics};
pub use types::{AggregateId, EventKey, ExpectedVersion, Sequence};
