//! This crate is a handy helper to share a MongoDB client within a process, for example, to share it among asynchronous request handlers.
//!
//! # Examples
//!
//! ```
//! use actix_web::{web, App, HttpServer};
//! use std::sync::{Arc, Mutex};
//! use shared_mongodb::{ClientHolder, database};
//! use mongodb::options::ClientOptions;
//!
//! #[actix_rt::main]
//! async fn main() -> std::io::Result<()> {
//!     let client_options = ClientOptions::parse("mongodb://root:password@localhost:12345").await;
//!     let client_holder = web::Data::new(Mutex::new(ClientHolder::new(client_options.unwrap())));
//!     HttpServer::new(move || {
//!         let app = App::new().app_data(client_holder.clone());
//!         return app;
//!     });
//!
//!     Ok(())
//! }
//!
//! async fn handler(data: web::Data<Mutex<ClientHolder>>) -> std::io::Result<()> {
//!     let db = database::get(&data, "My_Company");
//!     database::disconnect(&data);
//!
//!     let session = start_transaction(&data)?;
//!     commit_transaction(&mut session);
//!
//!     Ok(())
//! }
//! ```
mod client;
pub mod database;
pub use client::ClientHolder;
