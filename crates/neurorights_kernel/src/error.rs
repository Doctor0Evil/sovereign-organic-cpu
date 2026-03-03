//! Kernel Error Types
//!
//! Comprehensive error handling for neurorights kernel operations.

use core::fmt;
use crate::invariants::NeurorightType;

#[derive(Debug, Clone, PartialEq)]
pub enum KernelError {
    /// Invariant violation detected
    InvariantViolation {
        right_type: NeurorightType,
        details: String,
    },
    
    /// Invariant set unlocked without authorization
    InvariantSetUnlocked,
    
    /// Policy downgrade attempted (forbidden)
    PolicyDowngradeAttempted,
    
    /// Invalid DID signature
    InvalidSignature,
    
    /// Legal regime conflict unresolved
    LegalRegimeConflict { regimes: String },
    
    /// Audit log write failed
    AuditLogFailure { reason: String },
    
    /// EVOLVE token missing or invalid
    EvolveTokenMissing,
    
    /// Proposal missing required evidence bundle
    EvidenceBundleMissing,
}

impl fmt::Display for KernelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KernelError::InvariantViolation { right_type, details } => {
                write!(f, "Invariant violation [{}]: {}", right_type.to_invariant_string(), details)
            }
            KernelError::InvariantSetUnlocked => {
                write!(f, "Invariant set unlocked without authorization")
            }
            KernelError::PolicyDowngradeAttempted => {
                write!(f, "Policy downgrade attempted (forbidden by non-derogable invariants)")
            }
            KernelError::InvalidSignature => {
                write!(f, "Invalid DID signature")
            }
            KernelError::LegalRegimeConflict { regimes } => {
                write!(f, "Legal regime conflict unresolved: {}", regimes)
            }
            KernelError::AuditLogFailure { reason } => {
                write!(f, "Audit log write failed: {}", reason)
            }
            KernelError::EvolveTokenMissing => {
                write!(f, "EVOLVE token missing or invalid")
            }
            KernelError::EvidenceBundleMissing => {
                write!(f, "Proposal missing required 10-tag EvidenceBundle")
            }
        }
    }
}

pub type KernelResult<T> = Result<T, KernelError>;
