//! Sovereignty Error Types
//!
//! Comprehensive error handling for DID sovereignty operations.

use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum SovereigntyError {
    /// Invalid DID format
    InvalidDIDFormat {
        did: String,
        expected_prefix: String,
    },
    
    /// Missing public key
    MissingPublicKey,
    
    /// Identity not bound to DID
    IdentityNotBound,
    
    /// Identity not verified
    IdentityNotVerified,
    
    /// Invalid boot hash
    InvalidBootHash,
    
    /// Untrusted boot hash
    UntrustedBootHash,
    
    /// Boot chain violation
    BootChainViolation,
    
    /// No current boot hash
    NoCurrentBootHash,
    
    /// DID mismatch
    DIDMismatch {
        expected: String,
        actual: String,
    },
    
    /// Signing failed
    SigningFailed,
    
    /// Token already consumed
    TokenAlreadyConsumed,
    
    /// Token expired
    TokenExpired,
    
    /// Token revoked
    TokenRevoked,
    
    /// Token not active
    TokenNotActive,
    
    /// Token upgrade hash mismatch
    TokenUpgradeMismatch,
    
    /// Token not found
    TokenNotFound,
    
    /// Evidence bundle incomplete
    IncompleteEvidenceBundle {
        current: usize,
        required: usize,
    },
    
    /// Upgrade proposal rejected
    UpgradeRejected {
        reason: String,
    },
}

impl fmt::Display for SovereigntyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SovereigntyError::InvalidDIDFormat { did, expected_prefix } => {
                write!(f, "Invalid DID format: {} (expected prefix: {})", did, expected_prefix)
            }
            SovereigntyError::MissingPublicKey => {
                write!(f, "Missing public key")
            }
            SovereigntyError::IdentityNotBound => {
                write!(f, "Identity not bound to DID")
            }
            SovereigntyError::IdentityNotVerified => {
                write!(f, "Identity not verified")
            }
            SovereigntyError::InvalidBootHash => {
                write!(f, "Invalid boot hash")
            }
            SovereigntyError::UntrustedBootHash => {
                write!(f, "Untrusted boot hash")
            }
            SovereigntyError::BootChainViolation => {
                write!(f, "Boot chain violation detected")
            }
            SovereigntyError::NoCurrentBootHash => {
                write!(f, "No current boot hash")
            }
            SovereigntyError::DIDMismatch { expected, actual } => {
                write!(f, "DID mismatch: expected {}, got {}", expected, actual)
            }
            SovereigntyError::SigningFailed => {
                write!(f, "Signing failed")
            }
            SovereigntyError::TokenAlreadyConsumed => {
                write!(f, "EVOLVE token already consumed")
            }
            SovereigntyError::TokenExpired => {
                write!(f, "EVOLVE token expired")
            }
            SovereigntyError::TokenRevoked => {
                write!(f, "EVOLVE token revoked")
            }
            SovereigntyError::TokenNotActive => {
                write!(f, "EVOLVE token not active")
            }
            SovereigntyError::TokenUpgradeMismatch => {
                write!(f, "EVOLVE token upgrade hash mismatch")
            }
            SovereigntyError::TokenNotFound => {
                write!(f, "EVOLVE token not found")
            }
            SovereigntyError::IncompleteEvidenceBundle { current, required } => {
                write!(f, "Evidence bundle incomplete: {} / {}", current, required)
            }
            SovereigntyError::UpgradeRejected { reason } => {
                write!(f, "Upgrade proposal rejected: {}", reason)
            }
        }
    }
}

pub type SovereigntyResult<T> = Result<T, SovereigntyError>;
