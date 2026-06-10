pub mod approval;
pub mod auth;
pub mod candidate_mapping;
pub mod cli;
pub mod client;
pub mod errors;
pub mod fixture;
pub mod markers;
pub mod normalize;
#[path = "projection.rs"]
pub mod projection;
pub mod reconcile;
pub mod sources;
pub mod write;

pub use errors::{AdapterError, Result};
