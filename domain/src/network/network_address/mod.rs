mod dns_name;
mod protocol;

pub use dns_name::*;
pub use protocol::*;

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct NetworkAddress(Vec<NetworkAddressProtocol>);
