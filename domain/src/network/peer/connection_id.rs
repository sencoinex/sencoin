use crate::{Error, Result};
use serde::{de, Deserialize, Deserializer, Serialize};
use std::{fmt, str::FromStr};
use ulid::Ulid;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PeerConnectionId(Ulid);

impl PeerConnectionId {
    pub fn new() -> Self {
        Self(Ulid::new())
    }
}

impl AsRef<Ulid> for PeerConnectionId {
    fn as_ref(&self) -> &Ulid {
        &self.0
    }
}

impl From<Ulid> for PeerConnectionId {
    fn from(value: Ulid) -> Self {
        Self(value)
    }
}

impl Into<String> for PeerConnectionId {
    fn into(self) -> String {
        self.0.to_string()
    }
}

impl TryFrom<String> for PeerConnectionId {
    type Error = Error;
    fn try_from(s: String) -> Result<Self> {
        let id = Ulid::from_string(&s).map_err(|cause| Error::InvalidPeerConnectionId { cause })?;
        Ok(Self(id))
    }
}

impl FromStr for PeerConnectionId {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let id = Ulid::from_string(s).map_err(|cause| Error::InvalidPeerConnectionId { cause })?;
        Ok(Self(id))
    }
}

impl fmt::Display for PeerConnectionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Serialize for PeerConnectionId {
    fn serialize<S: serde::ser::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for PeerConnectionId {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        let id = Ulid::from_string(&s).map_err(de::Error::custom)?;
        Ok(Self(id))
    }
}
