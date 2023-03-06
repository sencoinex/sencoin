use super::DnsName;
use crate::network::PeerMessagingProtocolVersion;
use serde::{Deserialize, Serialize};

pub type Ipv4Addr = std::net::Ipv4Addr;
pub type Ipv6Addr = std::net::Ipv6Addr;
pub type TcpPort = u16;
pub type MemoryPort = u16;

/// A single protocol in the [`NetworkAddress`] protocol stack.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum NetworkAddressProtocol {
    Ip4(Ipv4Addr),
    Ip6(Ipv6Addr),
    Dns(DnsName),
    Dns4(DnsName),
    Dns6(DnsName),
    Tcp(TcpPort),
    Memory(MemoryPort),
    // TODO human-readable x25519::PublicKey is lower-case hex encoded
    // NoiseIK(x25519::PublicKey),
    Handshake(PeerMessagingProtocolVersion),
}
