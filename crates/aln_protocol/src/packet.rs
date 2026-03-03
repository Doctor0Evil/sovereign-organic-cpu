//! ALN Particle Structure
//!
//! Every message in the sovereign stack is an ALN Particle containing
//! DID signatures, evidence bundles, and corridor-safe semantics.

use alloc::string::String;
use alloc::vec::Vec;
use crate::error::{ProtocolError, ProtocolResult};
use crate::evidence_anchors;

/// Type of ALN particle
#[derive(Clone, Debug, PartialEq)]
pub enum ParticleType {
    /// Proposal for operation (external -> host)
    Proposal,
    /// Corridor-safe status export (host -> external)
    StatusExport,
    /// Guard decision credential (host -> external)
    GuardDecision,
    /// Audit log anchor (host -> chain)
    AuditAnchor,
}

/// Evidence header embedded in every particle
#[derive(Clone, Debug)]
pub struct EvidenceHeader {
    pub bundle_tags: Vec<String>, // 10-tag EvidenceBundle hex anchors
    pub roh_snapshot: f32,
    pub rod_snapshot: f32,
    pub lifeforce_band: String,
    pub timestamp: u64,
    pub evolve_token_hash: Option<String>,
}

impl EvidenceHeader {
    pub fn new() -> Self {
        Self {
            bundle_tags: Vec::new(),
            roh_snapshot: 0.0,
            rod_snapshot: 0.0,
            lifeforce_band: "Baseline".to_string(),
            timestamp: 0,
            evolve_token_hash: None,
        }
    }
    
    pub fn validate_completeness(&self) -> ProtocolResult<()> {
        if self.bundle_tags.len() < 10 {
            return Err(ProtocolError::IncompleteEvidenceBundle {
                current: self.bundle_tags.len(),
                required: 10,
            });
        }
        Ok(())
    }
    
    pub fn add_anchor(&mut self, anchor: &str) {
        if !self.bundle_tags.contains(&anchor.to_string()) {
            self.bundle_tags.push(anchor.to_string());
        }
    }
}

impl Default for EvidenceHeader {
    fn default() -> Self {
        Self::new()
    }
}

/// Complete ALN Particle
#[derive(Clone, Debug)]
pub struct ALNParticle {
    pub particle_type: ParticleType,
    pub sender_did: String,
    pub receiver_did: String,
    pub payload_hash: String,
    pub evidence_header: EvidenceHeader,
    pub signature: String,
    pub corridor_safe: bool,
}

impl ALNParticle {
    pub fn new_proposal(sender_did: String, receiver_did: String) -> ProtocolResult<Self> {
        Ok(Self {
            particle_type: ParticleType::Proposal,
            sender_did,
            receiver_did,
            payload_hash: String::new(),
            evidence_header: EvidenceHeader::new(),
            signature: String::new(),
            corridor_safe: false,
        })
    }
    
    pub fn mark_corridor_safe(&mut self, roh: f32, rod: f32, lifeforce: &str) -> ProtocolResult<()> {
        if roh > 0.3 || rod >= 1.0 || lifeforce == "HardStop" {
            return Err(ProtocolError::CorridorViolation {
                roh,
                rod,
                lifeforce: lifeforce.to_string(),
            });
        }
        
        self.evidence_header.roh_snapshot = roh;
        self.evidence_header.rod_snapshot = rod;
        self.evidence_header.lifeforce_band = lifeforce.to_string();
        self.corridor_safe = true;
        Ok(())
    }
    
    pub fn sign(&mut self, private_key: &str) -> ProtocolResult<()> {
        // In real implementation, perform cryptographic signature
        if private_key.is_empty() {
            return Err(ProtocolError::SigningFailed);
        }
        self.signature = "signed_hash_placeholder".to_string();
        Ok(())
    }
    
    pub fn verify_signature(&self, public_key: &str) -> ProtocolResult<bool> {
        // In real implementation, verify cryptographic signature
        if self.signature.is_empty() {
            return Ok(false);
        }
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evidence_anchors;
    
    #[test]
    fn test_particle_creation() {
        let particle = ALNParticle::new_proposal(
            "did:bostrom:sender".to_string(),
            "did:bostrom:receiver".to_string(),
        ).unwrap();
        
        assert_eq!(particle.particle_type, ParticleType::Proposal);
        assert!(!particle.corridor_safe);
    }
    
    #[test]
    fn test_evidence_header_completeness() {
        let mut header = EvidenceHeader::new();
        for i in 0..10 {
            header.add_anchor(&format!("0xtag{:02x}", i));
        }
        
        assert!(header.validate_completeness().is_ok());
    }
}
