//! API Gateway module for Tardigrade Git
//!
//! This module provides API Gateway functionality for routing requests
//! to different services and handling cross-cutting concerns.

pub mod config;
pub mod middleware;
pub mod router;

pub use config::*;
pub use middleware::*;
pub use router::*;
