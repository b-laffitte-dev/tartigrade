//! gRPC module for Tardigrade Git
//!
//! This module provides gRPC service implementation for the Git module.
//! It uses Tonic for gRPC and Prost for protocol buffers.

pub mod git;
pub mod server;

pub use git::*;
pub use server::*;
