//! Envelope Error Types
//!
//! Comprehensive error handling for biophysical envelope calculations.

use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum EnvelopeError {
    /// RoH threshold exceeded (RoH > 0.3)
    RoHThresholdExceeded { current: f32 },
    
    /// ROD HardStop triggered (ROD = 1.0)
    RodHardStop { current: f32 },
    
    /// LifeforceBand HardStop
    LifeforceHardStop { band: String },
    
    /// Eco-monotonicity violation
    EcoMonotonicityViolation { delta: f32 },
    
    /// Neurorights violation
    NeurorightsViolation { clause: String, details: String },
    
    /// Invalid telemetry data
    InvalidTelemetry { source: String },
    
    /// Calibration required
    CalibrationRequired { metric: String },
    
    /// Timestamp out of order
    TimestampOutOfOrder { expected: u64, actual: u64 },
}

impl fmt::Display for EnvelopeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnvelopeError::RoHThresholdExceeded { current } => {
                write!(f, "RoH threshold exceeded: {} > 0.3", current)
            }
            EnvelopeError::RodHardStop { current } => {
                write!(f, "ROD HardStop triggered: {}", current)
            }
            EnvelopeError::LifeforceHardStop { band } => {
                write!(f, "LifeforceBand HardStop: {}", band)
            }
            EnvelopeError::EcoMonotonicityViolation { delta } => {
                write!(f, "Eco-monotonicity violation: delta = {}", delta)
            }
            EnvelopeError::NeurorightsViolation { clause, details } => {
                write!(f, "Neurorights violation [{}]: {}", clause, details)
            }
            EnvelopeError::InvalidTelemetry { source } => {
                write!(f, "Invalid telemetry data from: {}", source)
            }
            EnvelopeError::CalibrationRequired { metric } => {
                write!(f, "Calibration required for: {}", metric)
            }
            EnvelopeError::TimestampOutOfOrder { expected, actual } => {
                write!(f, "Timestamp out of order: expected {}, got {}", expected, actual)
            }
        }
    }
}

pub type EnvelopeResult<T> = Result<T, EnvelopeError>;
