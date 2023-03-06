use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Clone, Copy, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub enum PeerProtocolId {
    // ConsensusRpcBcs = 0,
    // ConsensusDirectSendBcs = 1,
    TransactionPoolDirectSend = 2,
}
