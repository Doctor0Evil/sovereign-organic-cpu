//! Legacy Protocol Gateway Shims
//!
//! Translates corridor-safe ALN particles into legacy protocol frames (ROS2, BLE, CAN).
//! Legacy protocols are NEVER trusted; they only receive translated, vetted data.

use alloc::string::String;
use alloc::vec::Vec;
use crate::packet::{ALNParticle, ParticleType};
use crate::channel::{ChannelType, OuterDomain};
use crate::error::{ProtocolError, ProtocolResult};

/// Legacy protocol type
#[derive(Clone, Debug, PartialEq)]
pub enum LegacyProtocol {
    ROS2,
    BLE,
    CAN,
    USB,
}

/// Legacy shim trait
pub trait LegacyShim: Send + Sync {
    fn protocol_type(&self) -> LegacyProtocol;
    fn translate_to_legacy(&self, particle: ALNParticle) -> ProtocolResult<Vec<u8>>;
    fn translate_from_legacy(&self, data: Vec<u8>) -> ProtocolResult<ALNParticle>;
}

/// ROS2 compatibility shim
pub struct Ros2Shim {
    pub topic_prefix: String,
    pub qos_profile: String,
}

impl Ros2Shim {
    pub fn new(topic_prefix: String) -> Self {
        Self {
            topic_prefix,
            qos_profile: "SENSOR_DATA".to_string(),
        }
    }
}

impl LegacyShim for Ros2Shim {
    fn protocol_type(&self) -> LegacyProtocol {
        LegacyProtocol::ROS2
    }
    
    fn translate_to_legacy(&self, particle: ALNParticle) -> ProtocolResult<Vec<u8>> {
        // Only allow StatusExport or GuardDecision particles
        if particle.particle_type != ParticleType::StatusExport 
            && particle.particle_type != ParticleType::GuardDecision {
            return Err(ProtocolError::LegacyTranslationBlocked {
                reason: "Only export particles allowed to legacy protocols".to_string(),
            });
        }
        
        // In real implementation, serialize to ROS2 message format
        Ok(particle.payload_hash.as_bytes().to_vec())
    }
    
    fn translate_from_legacy(&self, data: Vec<u8>) -> ProtocolResult<ALNParticle> {
        // Legacy inputs are always treated as Proposals requiring guard vetting
        let mut particle = ALNParticle::new_proposal(
            "did:legacy:shim".to_string(),
            "did:bostrom:host".to_string(),
        )?;
        particle.payload_hash = String::from_utf8_lossy(&data).to_string();
        Ok(particle)
    }
}

/// BLE compatibility shim
pub struct BleShim {
    pub service_uuid: String,
    pub characteristic_uuid: String,
}

impl BleShim {
    pub fn new(service: String, characteristic: String) -> Self {
        Self {
            service_uuid: service,
            characteristic_uuid: characteristic,
        }
    }
}

impl LegacyShim for BleShim {
    fn protocol_type(&self) -> LegacyProtocol {
        LegacyProtocol::BLE
    }
    
    fn translate_to_legacy(&self, particle: ALNParticle) -> ProtocolResult<Vec<u8>> {
        // BLE has strict MTU limits; ensure particle fits
        if particle.payload_hash.len() > 512 {
            return Err(ProtocolError::LegacyMtuExceeded);
        }
        self.translate_to_legacy(particle)
    }
    
    fn translate_from_legacy(&self, data: Vec<u8>) -> ProtocolResult<ALNParticle> {
        // Same as ROS2: treat as proposal requiring vetting
        let mut particle = ALNParticle::new_proposal(
            "did:ble:shim".to_string(),
            "did:bostrom:host".to_string(),
        )?;
        particle.payload_hash = String::from_utf8_lossy(&data).to_string();
        Ok(particle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ros2_shim_translation() {
        let shim = Ros2Shim::new("aln_export".to_string());
        let mut particle = ALNParticle::new_proposal(
            "did:test".to_string(),
            "did:host".to_string(),
        ).unwrap();
        particle.particle_type = ParticleType::StatusExport;
        
        let result = shim.translate_to_legacy(particle);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_ros2_shim_blocks_proposal() {
        let shim = Ros2Shim::new("aln_export".to_string());
        let particle = ALNParticle::new_proposal(
            "did:test".to_string(),
            "did:host".to_string(),
        ).unwrap();
        // ParticleType is Proposal by default
        
        let result = shim.translate_to_legacy(particle);
        assert!(result.is_err());
    }
}
