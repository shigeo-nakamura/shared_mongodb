//! This crate is a handy helper to share a MongoDB client within a process, for example, to share it among asynchronous request handlers.
//!
//! # Examples
//!
//! ```
//! use actix_web::{web, App, HttpServer};
//! use std::sync::{Arc, Mutex};
//! use shared_mongodb::{ClientHolder, database};
//!
//! let client_holder = Arc::new(Mutex::new(ClientHolder::new("mongodb+srv://...")));
//! HttpServer::new(move || {
//!     let app = App::new().app_data(client_holder.clone());
//!     return app;
//! });
//! ```
mod client;
pub mod database;
pub use client::ClientHolder;
