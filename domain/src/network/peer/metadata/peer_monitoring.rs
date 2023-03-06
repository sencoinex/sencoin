use crate::network::{PeerId, PeerMetadata};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};

#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
pub struct PeerMonitoringMetadata {
    /// The average latency ping for the peer
    pub average_ping_latency: Option<Duration>,
    /// Connected peers and metadata
    pub connected_peers_and_metadata: Option<HashMap<PeerId, PeerMetadata>>,
}
