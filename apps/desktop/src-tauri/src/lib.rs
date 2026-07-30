//! Fanatir desktop library crate — T032 type-only Trusted Host skeleton.
//!
//! This library exposes a compile-time authority namespace only. Types grant no
//! permission, perform no I/O, and are not persistence, IPC, or WebView contracts.
//!
//! Project and Workspace relationship is intentionally undefined in T032.

#![forbid(unsafe_code)]

pub mod trusted_host;
