//! Guard Context Management
//!
//! Establishes the DID-bound context for all guard operations, ensuring
//! that every decision is anchored to the augmented citizen's identity.

use alloc::string::String;
use crate::error::{GuardError, GuardResult};
use crate::attestation::EvidenceBundle;

/// Unique identifier for OrganicCPU instance
#[derive(Clone, Debug)]
pub struct OrganicCpuId {
    pub instance_id: String,
    pub did: String,
    pub boot_hash: String,
}

impl OrganicCpuId {
    pub fn new(did: String, instance_id: String) -> Self {
        Self {
            did,
            instance_id,
            boot_hash: String::new(), // Populated at boot
        }
    }
    
    pub fn validate_boot_chain(&mut self, boot_hash: String) -> GuardResult<()> {
        if boot_hash.is_empty() {
            return Err(GuardError::InvalidBootChain);
        }
        self.boot_hash = boot_hash;
        Ok(())
    }
}

/// Context for all guard operations
#[derive(Clone, Debug)]
pub struct GuardContext {
    pub cpu_id: OrganicCpuId,
    pub evidence_bundle: EvidenceBundle,
    pub timestamp: u64,
    pub session_id: String,
}

impl GuardContext {
    pub fn new(did: String, cpu_instance_id: String) -> Self {
        Self {
            cpu_id: OrganicCpuId::new(did, cpu_instance_id),
            evidence_bundle: EvidenceBundle::empty(),
            timestamp: 0,
            session_id: String::new(),
        }
    }
    
    pub fn with_evidence_bundle(mut self, bundle: EvidenceBundle) -> Self {
        self.evidence_bundle = bundle;
        self
    }
    
    pub fn validate(&self) -> GuardResult<()> {
        if self.cpu_id.did.is_empty() {
            return Err(GuardError::MissingDID);
        }
        if self.cpu_id.instance_id.is_empty() {
            return Err(GuardError::InvalidCpuInstance);
        }
        if !self.evidence_bundle.is_complete() {
            return Err(GuardError::IncompleteEvidenceBundle);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_context_creation() {
        let ctx = GuardContext::new(
            "did:bostrom:bostrom18...".to_string(),
            "organic_cpu_001".to_string(),
        );
        assert_eq!(ctx.cpu_id.did, "did:bostrom:bostrom18...");
    }
    
    #[test]
    fn test_context_validation() {
        let ctx = GuardContext::new(
            "did:bostrom:bostrom18...".to_string(),
            "organic_cpu_001".to_string(),
        );
        // Should fail without evidence bundle
        assert!(ctx.validate().is_err());
    }
}
