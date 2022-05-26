use mongodb::Client;

/// Holds a MongoDB Client.
pub struct ClientHolder {
    /// Whether the client is connected to a MongoDB cluster
    pub connected: bool,
    /// A MongoDB Client
    pub client: Option<Client>,
    /// A MongoDB connection string
    pub mongodb_uri: String,
}

/// Create a new ClinetHolder.
///
/// # Arguments
///
/// * `mongodb_uri` - A MongoDB connection string
impl ClientHolder {
    pub fn new(mongodb_uri: &str) -> Self {
        Self {
            connected: false,
            client: None,
            mongodb_uri: mongodb_uri.to_string(),
        }
    }
}
