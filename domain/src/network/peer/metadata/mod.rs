mod connection;
mod peer_monitoring;

pub use connection::*;
pub use peer_monitoring::*;

use super::PeerConnectionState;
use serde::{Deserialize, Serialize};

/// A container holding all relevant metadata for the peer.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct PeerMetadata {
    pub connection_state: PeerConnectionState,
    pub connection_metadata: PeerConnectionMetadata,
    pub monitoring_metadata: PeerMonitoringMetadata,
}
