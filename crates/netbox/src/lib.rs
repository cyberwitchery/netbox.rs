#![doc = include_str!("../../../docs/client.md")]
#![warn(missing_docs)]
#![warn(clippy::all)]

mod client;
mod config;
mod error;
mod pagination;
mod query;
mod resource;

// api endpoint modules
/// circuits and provider resources.
pub mod circuits;
/// core endpoints and system resources.
pub mod core;
/// dcim endpoints.
pub mod dcim;
/// extras endpoints (tags, webhooks, scripts, custom fields).
pub mod extras;
/// ipam endpoints.
pub mod ipam;
/// plugin endpoints.
pub mod plugins;
/// schema endpoint.
pub mod schema;
/// status endpoint.
pub mod status;
/// tenancy endpoints.
pub mod tenancy;
/// users and auth endpoints.
pub mod users;
/// virtualization endpoints.
pub mod virtualization;
/// vpn endpoints.
pub mod vpn;
/// wireless endpoints.
pub mod wireless;

pub use client::Client;
pub use config::ClientConfig;
pub use error::{Error, Result};
pub use pagination::{Page, Paginator};
pub use query::QueryBuilder;
pub use resource::Resource;

/// generated openapi types and api functions.
pub mod openapi {
    pub use netbox_openapi::apis;
    pub use netbox_openapi::models;
}

// re-export the generated models for convenience
pub use netbox_openapi::models;
