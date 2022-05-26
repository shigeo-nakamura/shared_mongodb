use mongodb::options::ClientOptions;
use mongodb::Client;

/// Holds a MongoDB Client.
pub struct ClientHolder {
    /// Whether the client is connected to a MongoDB cluster
    pub connected: bool,
    /// A MongoDB Client
    pub client: Option<Client>,
    /// A MongoDB ClientOptions
    pub client_options: ClientOptions,
}

/// Create a new ClinetHolder.
///
/// # Arguments
///
/// * `client_options` - A MongoDB ClientOptions
impl ClientHolder {
    pub fn new(client_options: ClientOptions) -> Self {
        Self {
            connected: false,
            client: None,
            client_options: client_options,
        }
    }
}
