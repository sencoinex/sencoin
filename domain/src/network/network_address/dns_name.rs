use crate::{Error, Result};
use serde::{de, Deserialize, Deserializer, Serialize};
use std::{fmt, str::FromStr};

const MAX_DNS_NAME_SIZE: usize = 255;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct DnsName(String);

impl DnsName {
    fn validate(s: &str) -> Result<()> {
        if s.is_empty() {
            Err(Error::EmptyDnsNameString)
        } else if s.as_bytes().len() > MAX_DNS_NAME_SIZE {
            Err(Error::DnsNameTooLong(s.as_bytes().len()))
        } else if s.contains('/') {
            Err(Error::InvalidDnsNameCharacter)
        } else if !s.is_ascii() {
            Err(Error::DnsNameNonASCII(s.into()))
        } else {
            Ok(())
        }
    }
}

impl AsRef<str> for DnsName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Into<String> for DnsName {
    fn into(self) -> String {
        self.0
    }
}

impl TryFrom<String> for DnsName {
    type Error = Error;
    fn try_from(s: String) -> Result<Self> {
        Self::validate(&s).map(|_| DnsName(s))
    }
}

impl FromStr for DnsName {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Self::validate(s).map(|_| DnsName(s.to_owned()))
    }
}

impl fmt::Display for DnsName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<'de> Deserialize<'de> for DnsName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename = "DnsName")]
        struct DeserializeWrapper(String);
        let wrapper = DeserializeWrapper::deserialize(deserializer)?;
        let name = DnsName::try_from(wrapper.0).map_err(de::Error::custom)?;
        Ok(name)
    }
}
