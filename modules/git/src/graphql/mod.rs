//! GraphQL module for Tardigrade Git
//!
//! This module provides a GraphQL API for the Git module.
//! It uses async-graphql for schema definition and execution.

pub mod resolvers;
pub mod schema;

pub use resolvers::*;
pub use schema::*;
