use anyhow::Result;
use iroh::protocol::Router;
use iroh::Endpoint;
use iroh_gossip::net::Gossip;

#[tokio::main]
async fn main() -> Result<()> {
    // We've removed the `SecretKey::generate` method.
    // The `Endpoint` will generate a `SecretKey` for
    // you under the hood if you don't supply one.
    let endpoint = Endpoint::builder()
        .discovery_n0()
        .bind()
        .await?;

    println!("> our node id: {}", endpoint.node_id());

    // Build an instance of the gossip protocol
    // and add a clone of the endpoint we have built.
    // The gossip protocol will use the endpoint to
    // make connections.
    let gossip = Gossip::builder().spawn(endpoint.clone()).await?;

    // The Router is how we manage protocols on top
    // of the iroh endpoint. It handles all incoming
    // messages and routes them to the correct
    // protocol.
    let router = Router::builder(endpoint.clone())
        .accept(iroh_gossip::ALPN, gossip.clone())
        .spawn()
        .await?;

    // Cleanly shutdown the router.
    router.shutdown().await?;

    Ok(())
}