pub use super::{PeerNetworkId, PeerOperatorId};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Identifier of a node, represented as (network id, node operator id)
#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Deserialize, Serialize)]
pub struct PeerId {
    pub network_id: PeerNetworkId,
    pub operator_id: PeerOperatorId,
}

impl fmt::Debug for PeerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for PeerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.network_id, self.operator_id,)
    }
}
