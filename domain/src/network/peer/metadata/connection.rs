use super::super::PeerOperatorId;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct PeerConnectionMetadata {
    pub remote_peer_operator_id: PeerOperatorId,
}

impl fmt::Debug for PeerConnectionMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for PeerConnectionMetadata {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        /*
        write!(
            f,
            "[{},{},{},{},{:?},{:?}]",
            self.remote_peer_operator_id,
            self.addr,
            self.origin,
            self.messaging_protocol,
            self.application_protocols,
            self.role
        )
         */
        todo!()
    }
}
