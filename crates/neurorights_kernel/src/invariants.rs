//! Non-Derogable Invariants
//!
//! Defines the core neurorights that cannot be violated by any system operation.
//! These are compiled into machine-code checks within the OrganicCPU guard layer.

use alloc::string::String;
use alloc::vec::Vec;
use crate::error::{KernelError, KernelResult};

/// Types of neurorights protected by the kernel
#[derive(Clone, Debug, PartialEq)]
pub enum NeurorightType {
    CognitiveLiberty,
    MentalPrivacy,
    AugmentationContinuity,
    ProjectContinuity,
}

impl NeurorightType {
    pub fn to_invariant_string(&self) -> &'static str {
        match self {
            NeurorightType::CognitiveLiberty => "no_nonconsensual_modulation",
            NeurorightType::MentalPrivacy => "no_raw_neural_export",
            NeurorightType::AugmentationContinuity => "no_guard_removal",
            NeurorightType::ProjectContinuity => "no_downgrade_without_consent",
        }
    }
    
    pub fn from_invariant_string(s: &str) -> Option<Self> {
        match s {
            "no_nonconsensual_modulation" => Some(NeurorightType::CognitiveLiberty),
            "no_raw_neural_export" => Some(NeurorightType::MentalPrivacy),
            "no_guard_removal" => Some(NeurorightType::AugmentationContinuity),
            "no_downgrade_without_consent" => Some(NeurorightType::ProjectContinuity),
            _ => None,
        }
    }
}

/// Record of an invariant violation
#[derive(Clone, Debug)]
pub struct InvariantViolation {
    pub right_type: NeurorightType,
    pub description: String,
    pub timestamp: u64,
    pub proposal_hash: String,
    pub severity: u8, // 1-10, 10 being critical constitutional breach
}

impl InvariantViolation {
    pub fn new(right_type: NeurorightType, description: String, proposal_hash: String) -> Self {
        Self {
            right_type,
            description,
            timestamp: 0,
            proposal_hash,
            severity: 10, // Default to critical
        }
    }
}

/// Complete set of non-derogable invariants
#[derive(Clone, Debug)]
pub struct InvariantSet {
    pub rights: Vec<NeurorightType>,
    pub locked: bool,
}

impl InvariantSet {
    pub fn new() -> Self {
        Self {
            rights: vec![
                NeurorightType::CognitiveLiberty,
                NeurorightType::MentalPrivacy,
                NeurorightType::AugmentationContinuity,
                NeurorightType::ProjectContinuity,
            ],
            locked: true, // Cannot be unlocked without DID signature
        }
    }
    
    pub fn verify(&self, proposal: &crate::policy::PolicyShard) -> KernelResult<bool> {
        if !self.locked {
            return Err(KernelError::InvariantSetUnlocked);
        }
        
        for right in &self.rights {
            if !proposal.protects_right(right) {
                return Err(KernelError::InvariantViolation {
                    right_type: right.clone(),
                    details: format!("Proposal fails to protect {}", right.to_invariant_string()),
                });
            }
        }
        
        Ok(true)
    }
    
    pub fn is_locked(&self) -> bool {
        self.locked
    }
    
    pub fn lock(&mut self, did_signature: &str) -> KernelResult<()> {
        // In real implementation, verify DID signature here
        if did_signature.is_empty() {
            return Err(KernelError::InvalidSignature);
        }
        self.locked = true;
        Ok(())
    }
}

impl Default for InvariantSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy::PolicyShard;
    
    #[test]
    fn test_invariant_set_default() {
        let set = InvariantSet::default();
        assert_eq!(set.rights.len(), 4);
        assert!(set.is_locked());
    }
    
    #[test]
    fn test_invariant_verification() {
        let set = InvariantSet::default();
        let mut proposal = PolicyShard::new("test_proposal");
        proposal.add_right_protection(NeurorightType::CognitiveLiberty);
        
        // Should fail because not all rights are protected
        assert!(set.verify(&proposal).is_err());
    }
}
