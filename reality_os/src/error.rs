//! Integration Error Types
//!
//! Comprehensive error handling for reality.os integration operations.

use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum IntegrationError {
    /// Guard service not active
    GuardServiceNotActive,
    
    /// Invalid kernel module signature
    InvalidModuleSignature,
    
    /// Invalid kernel module hash
    InvalidModuleHash,
    
    /// Inner domain access denied
    InnerDomainAccessDenied {
        path: String,
    },
    
    /// Inner domain mmap blocked
    InnerDomainMmapBlocked,
    
    /// Raw socket blocked
    RawSocketBlocked,
    
    /// Device already registered
    DeviceAlreadyRegistered {
        path: String,
    },
    
    /// Invalid audit entry
    InvalidAuditEntry,
    
    /// Audit log write failed
    AuditLogWriteFailed,
    
    /// Chain anchor failed
    ChainAnchorFailed,
    
    /// DID binding missing
    DIDBindingMissing,
    
    /// Boot chain verification failed
    BootChainVerificationFailed,
    
    /// EVOLVE token validation failed
    EvolveTokenValidationFailed,
    
    /// Syscall wrapper initialization failed
    SyscallWrapperInitFailed,
}

impl fmt::Display for IntegrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IntegrationError::GuardServiceNotActive => {
                write!(f, "Guard service not active")
            }
            IntegrationError::InvalidModuleSignature => {
                write!(f, "Invalid kernel module signature")
            }
            IntegrationError::InvalidModuleHash => {
                write!(f, "Invalid kernel module hash")
            }
            IntegrationError::InnerDomainAccessDenied { path } => {
                write!(f, "Inner domain access denied: {}", path)
            }
            IntegrationError::InnerDomainMmapBlocked => {
                write!(f, "Inner domain mmap blocked")
            }
            IntegrationError::RawSocketBlocked => {
                write!(f, "Raw socket blocked for security")
            }
            IntegrationError::DeviceAlreadyRegistered { path } => {
                write!(f, "Device already registered: {}", path)
            }
            IntegrationError::InvalidAuditEntry => {
                write!(f, "Invalid audit entry")
            }
            IntegrationError::AuditLogWriteFailed => {
                write!(f, "Audit log write failed")
            }
            IntegrationError::ChainAnchorFailed => {
                write!(f, "Chain anchor failed")
            }
            IntegrationError::DIDBindingMissing => {
                write!(f, "DID binding missing")
            }
            IntegrationError::BootChainVerificationFailed => {
                write!(f, "Boot chain verification failed")
            }
            IntegrationError::EvolveTokenValidationFailed => {
                write!(f, "EVOLVE token validation failed")
            }
            IntegrationError::SyscallWrapperInitFailed => {
                write!(f, "Syscall wrapper initialization failed")
            }
        }
    }
}

pub type IntegrationResult<T> = Result<T, IntegrationError>;
