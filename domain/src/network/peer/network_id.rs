use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

// TODO define peer_network_id options
#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum PeerNetworkId {}

impl PeerNetworkId {
    pub fn as_str(&self) -> &str {
        todo!()
    }
}

impl AsRef<str> for PeerNetworkId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Default for PeerNetworkId {
    fn default() -> Self {
        todo!()
    }
}

impl fmt::Debug for PeerNetworkId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl fmt::Display for PeerNetworkId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for PeerNetworkId {
    fn serialize<S: Serializer>(&self, _serializer: S) -> Result<S::Ok, S::Error> {
        todo!()
    }
}

impl<'de> Deserialize<'de> for PeerNetworkId {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}
