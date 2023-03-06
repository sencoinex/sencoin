use super::{PeerId, PeerMetadata, PeerProtocolId};
use std::collections::HashMap;

pub trait PeerConnectionRepository: Send + Sync + 'static {
    type Error: ::std::error::Error + Send + Sync + 'static;

    /// Returns all peer ids.
    /// Note: this will return disconnected and unhealthy peers.
    fn get_all_peers(&self) -> Result<Vec<PeerId>, Self::Error>;

    /// Returns all connected peers that support at least one of
    /// the given protocols.
    fn get_connected_supported_peers(
        &self,
        protocol_ids: &[PeerProtocolId],
    ) -> Result<Vec<PeerId>, Self::Error>;

    /// Returns metadata for all peers currently connected to the node
    fn get_connected_peers_and_metadata(
        &self,
    ) -> Result<HashMap<PeerId, PeerMetadata>, Self::Error>;
}
