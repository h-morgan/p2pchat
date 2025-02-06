use anyhow::Result;
use iroh::protocol::Router;
use iroh::Endpoint;
use iroh_gossip::{net::Gossip, proto::TopicId};

#[tokio::main]
async fn main() -> Result<()> {
    let endpoint = Endpoint::builder().discovery_n0().bind().await?;

    println!("> our node id: {}", endpoint.node_id());
    let gossip = Gossip::builder().spawn(endpoint.clone()).await?;

    let router = Router::builder(endpoint.clone())
        .accept(iroh_gossip::ALPN, gossip.clone())
        .spawn()
        .await?;

    // Create a new topic.
    let id = TopicId::from_bytes(rand::random());
    let node_ids = vec![];

    // Subscribe to the topic.
    // Since the `node_ids` list is empty, we will
    // subscribe to the topic, but not attempt to
    // connect to any other nodes.
    let topic = gossip.subscribe(id, node_ids)?;

    // `split` splits the topic into the `GossipSender`
    // and `GossipReceiver` portions
    let (sender, _receiver) = topic.split();

    // Broadcast a messsage to the topic.
    // Since no one else is apart of this topic,
    // this message is currently going out to no one.
    sender.broadcast("sup".into()).await?;

    router.shutdown().await?;

    Ok(())
}
