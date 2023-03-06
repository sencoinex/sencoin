use serde::{Deserialize, Serialize};

/// The current connection state of a peer
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Deserialize, Serialize)]
pub enum PeerConnectionState {
    Connected,
    Disconnecting,
}
