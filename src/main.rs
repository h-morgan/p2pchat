use anyhow::Result;
use iroh::{Endpoint, SecretKey};

#[tokio::main]
async fn main() -> Result<()> {
    // Generate a secret key. This is the source of
    // identity for your node. If you want to have
    // the same identity each time you open the app,
    // you would need to store and load it each time.
    let secret_key = SecretKey::generate(rand::rngs::OsRng);

    // Create an endpoint.
    let endpoint = Endpoint::builder()
        // Pass in your secret key. If you don't pass
        // in a secret key a new one will be generated
        // for you each time.
        .secret_key(secret_key)
        // Enable n0 discovery. This allows you to 
        // dial by `NodeId`, and allows you to be
        // dialed by `NodeId`.
        .discovery_n0()
        // Bind the endpoint to the socket.
        .bind()
        .await?;

    println!("> our node id: {}", endpoint.node_id());

    Ok(())
}
