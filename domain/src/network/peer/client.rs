use super::{PeerId, PeerMessage};
use crate::network::NetworkAddress;
use async_trait::async_trait;
use std::time::Duration;

pub struct PeerIdWithAddress((PeerId, NetworkAddress));

pub type DisconnectReason = String;

#[async_trait]
pub trait PeerClient<Message: PeerMessage>: Clone + Send + Sync {
    type Error: ::std::error::Error + Send + Sync + 'static;

    /// Adds the given peer list to the set of discovered peers
    /// that can potentially be dialed for future connections.
    async fn add_peers_to_discovery(&self, peers: &[PeerIdWithAddress]) -> Result<(), Self::Error>;

    /// Requests that the network connection for the specified peer
    /// is disconnected.
    async fn disconnect_from_peer(
        &self,
        peer_id: PeerId,
        reason: Option<DisconnectReason>,
    ) -> Result<(), Self::Error>;

    /// Returns a list of available peers (i.e., those that are
    /// currently connected and support the relevant protocols
    /// for the client).
    fn get_available_peers(&self) -> Result<Vec<PeerId>, Self::Error>;

    /// Sends the given message to the specified peer. Note: this
    /// Note: this method does not guarantee message delivery or handle responses.
    fn send_to_peer(&self, _message: Message, peer_id: PeerId) -> Result<(), Self::Error>;

    /// Sends the given message to each peer in the specified peer list.
    /// Note: this method does not guarantee message delivery or handle responses.
    fn send_to_peers(&self, _message: Message, peer_ids: &[PeerId]) -> Result<(), Self::Error>;

    /// Sends the given message to the specified peer with the corresponding
    /// timeout. Awaits a response from the peer, or hits the timeout
    /// (whichever occurs first).
    async fn send_to_peer_rpc(
        &self,
        message: Message,
        rpc_timeout: Duration,
        peer_id: PeerId,
    ) -> Result<Message, Self::Error>;
}
