//! # netbox
//!
//! An ergonomic Rust client for the NetBox 4.x REST API.
//!
//! ## Features
//!
//! - Strongly typed API using generated bindings from NetBox OpenAPI schema
//! - Token-based authentication
//! - Automatic pagination handling
//! - Flexible query filtering
//! - Configurable timeouts and retries
//! - Optional tracing support
//!
//! ## Quick Start
//!
//! ```no_run
//! use netbox::{Client, ClientConfig};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let config = ClientConfig::new("https://netbox.example.com", "your-api-token");
//! let client = Client::new(config)?;
//!
//! // List devices
//! // let devices = client.dcim().devices().list().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Version Compatibility
//!
//! This crate version tracks NetBox major.minor versions:
//! - `netbox 4.2.x` targets NetBox 4.2.x
//! - `netbox 4.3.x` targets NetBox 4.3.x
//!
//! Patch versions may include bug fixes and improvements to the client library.

#![warn(missing_docs)]
#![warn(clippy::all)]

mod client;
mod config;
mod error;
mod pagination;
mod query;
mod resource;

// API endpoint modules
pub mod circuits;
pub mod core;
pub mod dcim;
pub mod extras;
pub mod ipam;
pub mod plugins;
pub mod schema;
pub mod status;
pub mod tenancy;
pub mod users;
pub mod virtualization;
pub mod vpn;
pub mod wireless;

pub use client::Client;
pub use config::ClientConfig;
pub use error::{Error, Result};
pub use pagination::{Page, Paginator};
pub use query::QueryBuilder;
pub use resource::Resource;

/// Generated OpenAPI types and API functions.
pub mod openapi {
    pub use netbox_openapi::apis;
    pub use netbox_openapi::models;
}

// Re-export the generated models for convenience
pub use netbox_openapi::models;
