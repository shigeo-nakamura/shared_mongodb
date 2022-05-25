//! This crate is a handy helper to share a MongoDB client within a process, for example, to share it among asynchronous request handlers.
//!
//! # Examples
//!
//! ```
//! // in main.rs
//! use std::sync::{Arc, Mutex};
//! use shared_mongodb::{ClientHolder, database};
//! use actix_web::{web, App, HttpServer};
//!
//! let client_holder = Arc::new(Mutex::new(ClientHolder::new("mongodb+srv://...")));
//! HttpServer::new(move || {
//!     let app = App::new().app_data(client_holder.clone());
//!     return app;
//! })
//!
//! // in handler.rs
//! pub async fn search(
//!     form: web::Query<FormData>,
//!     data: web::Data<Mutex<ClientHolder>>) -> Result<HttpResponse> {
//!     let db = database::get(&data).await;
//!     ...
//! }
//! ```
use mongodb::Client;
pub mod database;

/// Holds a MongoDB Client.
pub struct ClientHolder {
    /// Whether the client is connected to a MongoDB cluster
    pub connected: bool,
    /// A MongoDB Client
    pub client: Option<Client>,
    /// A MongoDB connection string
    pub mongodb_uri: Option<String>,
}

/// Create a new ClinetHolder.
///
/// # Arguments
///
/// * `mongodb_uri` - A MongoDB connection string
impl ClientHolder {
    pub fn new(mongodb_uri: &String) -> Self {
        Self {
            connected: false,
            client: None,
            mongodb_uri: Some(mongodb_uri.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
