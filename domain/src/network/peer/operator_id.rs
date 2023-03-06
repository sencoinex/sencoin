use serde::{Deserialize, Serialize};
use std::{fmt, hash::Hash};

// TODO define peer_operator_id struct(basically operator account public key)
#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Deserialize, Serialize)]
pub struct PeerOperatorId {}

impl fmt::Display for PeerOperatorId {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
