use serde::{de::DeserializeOwned, Serialize};

/// A simple definition of the message type used by p2p protocol.
pub trait PeerMessage: Clone + DeserializeOwned + Serialize + Send + Sync + 'static {}
impl<T: Clone + DeserializeOwned + Serialize + Send + Sync + 'static> PeerMessage for T {}
