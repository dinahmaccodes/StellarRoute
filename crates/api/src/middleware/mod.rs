//! API middleware

pub mod deprecation;
pub mod rate_limit;
pub mod tracing;
pub mod validation;

pub use deprecation::{legacy_route_deprecation, LEGACY_ROUTE_SUNSET, VERSIONING_GUIDE_URL};
pub use rate_limit::{EndpointConfig, RateLimitConfig, RateLimitLayer};
pub use tracing::{extract_context_from_headers, inject_context_to_map, trace_layer};
pub use validation::ValidatedQuoteRequest;
