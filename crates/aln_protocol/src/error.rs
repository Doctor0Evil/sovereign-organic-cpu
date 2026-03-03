//! Protocol Error Types
//!
//! Comprehensive error handling for ALN networking operations.

use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ProtocolError {
    /// Corridor violation detected (RoH/ROD/Lifeforce)
    CorridorViolation {
        roh: f32,
        rod: f32,
        lifeforce: String,
    },
    
    /// Evidence bundle incomplete (requires 10 tags)
    IncompleteEvidenceBundle {
        current: usize,
        required: usize,
    },
    
    /// Legacy translation blocked for security
    LegacyTranslationBlocked {
        reason: String,
    },
    
    /// Legacy protocol MTU exceeded
    LegacyMtuExceeded,
    
    /// DID signature verification failed
    SignatureVerificationFailed,
    
    /// Signing failed (missing key)
    SigningFailed,
    
    /// Channel access denied (Inner/Outer domain violation)
    ChannelAccessDenied {
        channel_type: String,
        requested_domain: String,
    },
    
    /// Routing verdict rejected
    RoutingRejected {
        verdict: String,
    },
    
    /// Packet timestamp out of order
    TimestampOutOfOrder,
}

impl fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProtocolError::CorridorViolation { roh, rod, lifeforce } => {
                write!(f, "Corridor violation: RoH={}, ROD={}, Lifeforce={}", roh, rod, lifeforce)
            }
            ProtocolError::IncompleteEvidenceBundle { current, required } => {
                write!(f, "Evidence bundle incomplete: {} / {}", current, required)
            }
            ProtocolError::LegacyTranslationBlocked { reason } => {
                write!(f, "Legacy translation blocked: {}", reason)
            }
            ProtocolError::LegacyMtuExceeded => {
                write!(f, "Legacy protocol MTU exceeded")
            }
            ProtocolError::SignatureVerificationFailed => {
                write!(f, "DID signature verification failed")
            }
            ProtocolError::SigningFailed => {
                write!(f, "Signing failed (missing key)")
            }
            ProtocolError::ChannelAccessDenied { channel_type, requested_domain } => {
                write!(f, "Channel access denied: {} requested {}", channel_type, requested_domain)
            }
            ProtocolError::RoutingRejected { verdict } => {
                write!(f, "Routing rejected: {}", verdict)
            }
            ProtocolError::TimestampOutOfOrder => {
                write!(f, "Packet timestamp out of order")
            }
        }
    }
}

pub type ProtocolResult<T> = Result<T, ProtocolError>;
