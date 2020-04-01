//! Unofficial client for InfluxDB v2
//!
//! This crate is under development.
//!
//! At this moment this allow you:
//! - write [`Batch`] from crate influxdb-line-protocol
//!
//! See examples folder for quick start.
//!
//! [`Batch`]:influxdb_line_protocol::Batch

// #![deny(missing_docs)]

pub mod bucket;
mod client;
pub mod error;
mod id;
mod timestamp;
pub mod write;

pub use client::Client;
