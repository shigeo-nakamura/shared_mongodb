use crate::ClientHolder;
use mongodb::options::ClientOptions;
use mongodb::{Client, Database};
use std::error;
use std::sync::{Arc, Mutex};

/// Get the handle of the database.
/// If the client is not connected to the MongoDB cluster, establish a new connection and return the handle of the database.
///
/// # Arguments
///
/// * `data` - A ClinetHolder
/// * `database_name` - The name of a Database
pub async fn get(
    data: &Arc<Mutex<ClientHolder>>,
    database_name: &str,
) -> Result<Database, Box<dyn error::Error>> {
    let client_holder = data.lock().unwrap();
    if client_holder.connected {
        let db = client_holder
            .client
            .as_ref()
            .unwrap()
            .database(database_name);
        return Ok(db);
    }
    let client_options = client_holder.client_options.clone();
    drop(client_holder);

    let new_client = connect(client_options).await?;

    let mut client_holder = data.lock().unwrap();
    client_holder.client = Some(new_client.clone());
    client_holder.connected = true;
    drop(client_holder);

    let db = new_client.database(database_name);
    Ok(db)
}

/// Disconnect the connection to the MongoDB cluster.
///
/// # Arguments
///
/// * `data` - A ClinetHolder
pub fn disconnect(data: &Arc<Mutex<ClientHolder>>) {
    let mut client_holder = data.lock().unwrap();
    client_holder.connected = false;
}

/// Establish a new connection to the MongoDB cluster.
///
/// # Arguments
///
/// * `mongodb_uri` - A MongoDB connection string
async fn connect(client_options: ClientOptions) -> Result<Client, Box<dyn error::Error>> {
    let new_client = Client::with_options(client_options)?;
    Ok(new_client)
}
