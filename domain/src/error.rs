use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("dns name cannot be empty")]
    EmptyDnsNameString,
    #[error("dns name cannot contain '/' characters")]
    InvalidDnsNameCharacter,
    #[error("dns name contains non-ASCII characters: {0}")]
    DnsNameNonASCII(String),
    #[error("dns name is too long: len: {0} bytes, max len: 255 bytes")]
    DnsNameTooLong(usize),
    #[error("peer connection id must be in \"Universally Unique Lexicographically Sortable Identifier\" format: {cause}")]
    InvalidPeerConnectionId { cause: ulid::DecodeError },
}
