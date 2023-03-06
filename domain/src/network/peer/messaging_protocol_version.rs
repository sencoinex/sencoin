use serde::{Deserialize, Serialize};
use std::fmt;

const V1: &'static str = "V1";

/// These should be listed from old to new, old having the smallest value.
/// We derive [`PartialOrd`] since nodes need to find highest intersecting protocol version.
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Deserialize, Serialize)]
pub enum PeerMessagingProtocolVersion {
    V1 = 0,
}

impl PeerMessagingProtocolVersion {
    fn as_str(&self) -> &str {
        match self {
            Self::V1 => V1,
        }
    }
}

impl fmt::Debug for PeerMessagingProtocolVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for PeerMessagingProtocolVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
