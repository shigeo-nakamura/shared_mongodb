use crate::ClientHolder;
use mongodb::options::ClientOptions;
use mongodb::{Client, ClientSession, Database};
use std::error;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Get the handle of the client.
/// If the client is not connected to the MongoDB cluster, establish a new connection and return the handle of the client.
///
/// # Arguments
///
/// * `data` - A ClinetHolder
/// * `database_name` - The name of a Database
async fn get_client(data: &Arc<Mutex<ClientHolder>>) -> Result<Client, Box<dyn error::Error>> {
    let client_holder = data.lock().await;
    if client_holder.connected {
        let client = client_holder.client.clone().unwrap();
        return Ok(client);
    }
    let client_options = client_holder.client_options.clone();
    drop(client_holder);

    let new_client = connect(client_options).await?;

    let mut client_holder = data.lock().await;
    client_holder.client = Some(new_client.clone());
    client_holder.connected = true;
    drop(client_holder);

    Ok(new_client)
}

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
    let db = get_client(data).await?.database(database_name);
    Ok(db)
}

/// Disconnect the connection to the MongoDB cluster.
///
/// # Arguments
///
/// * `data` - A ClinetHolder
pub async fn disconnect(data: &Arc<Mutex<ClientHolder>>) {
    let mut client_holder = data.lock().await;
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

/// Start the transaction in a new session.
///
/// # Arguments
///
/// * `data` - A ClinetHolder
pub async fn start_transaction(
    data: &Arc<Mutex<ClientHolder>>,
) -> Result<ClientSession, Box<dyn error::Error>> {
    let client = get_client(data).await?;
    let mut session = client.start_session(None).await?;
    session.start_transaction(None).await?;
    Ok(session)
}

/// Commit the transaction in the given session.
///
/// # Arguments
///
/// * `data` - A ClinetHolder
pub async fn commit_transaction(session: &mut ClientSession) -> Result<(), Box<dyn error::Error>> {
    session.commit_transaction().await?;
    Ok(())
}

/// Abort the transaction in the given session.
///
/// # Arguments
///
/// * `data` - A ClinetHolder
pub async fn abort_transaction(session: &mut ClientSession) -> Result<(), Box<dyn error::Error>> {
    session.abort_transaction().await?;
    Ok(())
}
