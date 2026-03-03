//! Guard Error Types
//!
//! Comprehensive error handling for all guard operations with
//! neurorights-violation tracking and audit logging.

use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum GuardError {
    /// RoH threshold exceeded (RoH > 0.3)
    RohThresholdExceeded { current: f32, max: f32 },
    
    /// ROD HardStop triggered (ROD = 1.0)
    RodHardStop { current: f32 },
    
    /// LifeforceBand violation
    LifeforceViolation { band: String, reason: String },
    
    /// Neurorights invariant violated
    NeurorightsViolation { clause: String, details: String },
    
    /// Evidence bundle incomplete or invalid
    IncompleteEvidenceBundle,
    
    /// Missing DID binding
    MissingDID,
    
    /// Invalid OrganicCPU instance
    InvalidCpuInstance,
    
    /// Boot chain validation failed
    InvalidBootChain,
    
    /// Device access denied (inner domain protected)
    DeviceAccessDenied { device_id: String, domain: String },
    
    /// Network export blocked (raw neural data)
    NetworkExportBlocked { data_type: String },
    
    /// EVOLVE token validation failed
    EvolveTokenInvalid,
    
    /// Upgrade proposal rejected
    UpgradeRejected { reason: String },
    
    /// Cryptographic attestation failed
    AttestationFailed { signature: String },
    
    /// Eco-monotonicity violation
    EcoMonotonicityViolation { delta: f32 },
}

impl fmt::Display for GuardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GuardError::RohThresholdExceeded { current, max } => {
                write!(f, "RoH threshold exceeded: {} > {}", current, max)
            }
            GuardError::RodHardStop { current } => {
                write!(f, "ROD HardStop triggered: {}", current)
            }
            GuardError::LifeforceViolation { band, reason } => {
                write!(f, "LifeforceBand violation [{}]: {}", band, reason)
            }
            GuardError::NeurorightsViolation { clause, details } => {
                write!(f, "Neurorights violation [{}]: {}", clause, details)
            }
            GuardError::IncompleteEvidenceBundle => {
                write!(f, "Evidence bundle incomplete (requires 10 tags)")
            }
            GuardError::MissingDID => {
                write!(f, "Missing DID binding")
            }
            GuardError::InvalidCpuInstance => {
                write!(f, "Invalid OrganicCPU instance")
            }
            GuardError::InvalidBootChain => {
                write!(f, "Boot chain validation failed")
            }
            GuardError::DeviceAccessDenied { device_id, domain } => {
                write!(f, "Device access denied: {} [{}]", device_id, domain)
            }
            GuardError::NetworkExportBlocked { data_type } => {
                write!(f, "Network export blocked for raw neural data: {}", data_type)
            }
            GuardError::EvolveTokenInvalid => {
                write!(f, "EVOLVE token validation failed")
            }
            GuardError::UpgradeRejected { reason } => {
                write!(f, "Upgrade proposal rejected: {}", reason)
            }
            GuardError::AttestationFailed { signature } => {
                write!(f, "Cryptographic attestation failed: {}", signature)
            }
            GuardError::EcoMonotonicityViolation { delta } => {
                write!(f, "Eco-monotonicity violation: delta = {}", delta)
            }
        }
    }
}

pub type GuardResult<T> = Result<T, GuardError>;
