use anyhow::{bail, Result};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EdgeId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EpisodeId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProviderId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CapabilityId(pub String);

impl NodeId {
    pub fn parse(value: &str) -> Result<Self> {
        if !value.starts_with("node:") || value.split(':').count() < 3 {
            bail!("invalid NodeId: {}", value);
        }
        Ok(Self(value.to_string()))
    }
}

impl EdgeId {
    pub fn parse(value: &str) -> Result<Self> {
        if !value.starts_with("edge:") || value.split(':').count() < 4 {
            bail!("invalid EdgeId: {}", value);
        }
        Ok(Self(value.to_string()))
    }
}

impl EpisodeId {
    pub fn parse(value: &str) -> Result<Self> {
        if !value.starts_with("episode:") || value.split(':').count() < 3 {
            bail!("invalid EpisodeId: {}", value);
        }
        Ok(Self(value.to_string()))
    }
}

impl ProviderId {
    pub fn parse(value: &str) -> Result<Self> {
        if !value.starts_with("provider:") || value.split(':').count() < 2 {
            bail!("invalid ProviderId: {}", value);
        }
        Ok(Self(value.to_string()))
    }
}

impl CapabilityId {
    pub fn parse(value: &str) -> Result<Self> {
        if !value.starts_with("cap:") || value.split(':').count() < 2 {
            bail!("invalid CapabilityId: {}", value);
        }
        Ok(Self(value.to_string()))
    }
}
