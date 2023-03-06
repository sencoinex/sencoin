use serde::{Deserialize, Serialize};
use std::fmt;

const INBOUND: &'static str = "inbound";
const OUTBOUND: &'static str = "outbound";

/// Origin of how a Connection was established.
#[derive(Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionOrigin {
    /// `Inbound` indicates that we are the listener for this connection.
    Inbound,
    /// `Outbound` indicates that we are the dialer for this connection.
    Outbound,
}

impl ConnectionOrigin {
    pub fn as_str(self) -> &'static str {
        match self {
            ConnectionOrigin::Inbound => INBOUND,
            ConnectionOrigin::Outbound => OUTBOUND,
        }
    }
}

impl fmt::Debug for ConnectionOrigin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl fmt::Display for ConnectionOrigin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
