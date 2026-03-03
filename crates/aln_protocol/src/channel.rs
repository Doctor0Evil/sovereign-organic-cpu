//! Sovereign Channel Types
//!
//! Enforces strict separation between INNER domain (neural) and OUTER domain (network).
//! No raw data crosses from INNER to OUTER without guard vetting.

use alloc::string::String;
use crate::packet::ALNParticle;
use crate::error::{ProtocolError, ProtocolResult};

/// Domain classification
#[derive(Clone, Debug, PartialEq)]
pub enum DomainType {
    Inner, // BCI/EEG/nanoswarm (host-only)
    Outer, // Network/audit (exportable)
}

/// Channel type enumeration
#[derive(Clone, Debug, PartialEq)]
pub enum ChannelType {
    NeuroIntraHost,      // INNER: BCI/EEG in (never exposed)
    NeuroOuterCorridor,  // OUTER: RoH/ROD/Lifeforce scalars only
    GuardDecision,       // OUTER: DID-signed guard verdicts
    LegacyShim,          // OUTER: Translated ROS/BLE frames
}

impl ChannelType {
    pub fn domain(&self) -> DomainType {
        match self {
            ChannelType::NeuroIntraHost => DomainType::Inner,
            _ => DomainType::Outer,
        }
    }
    
    pub fn allows_raw_neural_data(&self) -> bool {
        match self {
            ChannelType::NeuroIntraHost => true,
            _ => false,
        }
    }
}

/// Sovereign channel trait
pub trait SovereignChannel: Send + Sync {
    fn channel_type(&self) -> ChannelType;
    fn send(&self, particle: ALNParticle) -> ProtocolResult<()>;
    fn receive(&self) -> ProtocolResult<ALNParticle>;
    fn is_inner_domain(&self) -> bool {
        self.channel_type().domain() == DomainType::Inner
    }
}

/// Inner domain channel (host-only)
pub struct InnerDomain {
    pub channel_id: String,
    pub device_tag: String,
}

impl InnerDomain {
    pub fn new(channel_id: String, device_tag: String) -> Self {
        Self { channel_id, device_tag }
    }
}

/// Outer domain channel (network-exportable)
pub struct OuterDomain {
    pub channel_id: String,
    pub did_bound: String,
    pub encryption_layer: String,
}

impl OuterDomain {
    pub fn new(channel_id: String, did_bound: String) -> Self {
        Self {
            channel_id,
            did_bound,
            encryption_layer: "AES-256-GCM".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_channel_domain_separation() {
        let inner = ChannelType::NeuroIntraHost;
        let outer = ChannelType::NeuroOuterCorridor;
        
        assert_eq!(inner.domain(), DomainType::Inner);
        assert_eq!(outer.domain(), DomainType::Outer);
        assert!(!outer.allows_raw_neural_data());
    }
}
