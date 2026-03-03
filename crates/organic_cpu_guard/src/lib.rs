//! OrganicCPU Guard Core
//!
//! This crate implements the primary enforcement surface for all neural interactions,
//! BCI routing, nanoswarm control, and XR rendering. Every operation must pass
//! through these guards before accessing inner-domain devices or exporting data.
//!
//! # Security Model
//!
//! - Inner Domain: BCI/EEG/nanoswarm channels (never exposed externally)
//! - Outer Domain: Corridor-safe scalars only (RoH, BCI bands, LifeforceBand)
//! - Proposal-Only: All external interactions are signed proposals
//!
//! # Example
//!
//! ```rust
//! use organic_cpu_guard::{GuardContext, NeurorightsGuard, BiophysicalEnvelope};
//!
//! let ctx = GuardContext::new(did, organic_cpu_id);
//! let guard = NeurorightsGuard::load_from_aln_shard(&ctx)?;
//!
//! if guard.check_roh(envelope)? <= 0.3 {
//!     guard.approve_operation(operation)?;
//! } else {
//!     guard.reject_with_hardstop("RoH threshold exceeded")?;
//! }
//! ```

#![no_std]
#![cfg_attr(feature = "formal-verification", feature(custom_attribute))]
#![warn(missing_docs)]
#![warn(clippy::all)]
#![forbid(unsafe_code)]

extern crate alloc;

pub mod context;
pub mod guard;
pub mod scheduler;
pub mod router;
pub mod attestation;
pub mod error;

pub use context::GuardContext;
pub use guard::{NeurorightsGuard, BiophysicalGuard, DeviceGuard};
pub use scheduler::{MlPassSchedule, CognitiveLoadEnvelope};
pub use router::{NeuralRope, UpgradeDescriptor};
pub use attestation::{GuardDecisionCredential, EvidenceBundle};
pub use error::{GuardError, GuardResult};

/// Core guard trait that all enforcement layers must implement
pub trait OrganicCpuGuard: Send + Sync {
    /// Check if operation satisfies RoH ≤ 0.3 constraint
    fn check_roh(&self, envelope: &BiophysicalEnvelope) -> GuardResult<f32>;
    
    /// Check if operation satisfies ROD < 1.0 constraint
    fn check_rod(&self, envelope: &BiophysicalEnvelope) -> GuardResult<f32>;
    
    /// Check if operation satisfies LifeforceBand constraints
    fn check_lifeforce(&self, envelope: &BiophysicalEnvelope) -> GuardResult<bool>;
    
    /// Check if operation satisfies neurorights invariants
    fn check_neurorights(&self, operation: &UpgradeDescriptor) -> GuardResult<bool>;
    
    /// Check if operation satisfies eco-monotonicity
    fn check_eco_monotonicity(&self, operation: &UpgradeDescriptor) -> GuardResult<bool>;
    
    /// Approve or reject operation with cryptographic attestation
    fn evaluate(&self, operation: &UpgradeDescriptor) -> GuardResult<GuardDecisionCredential>;
}

/// Version information for guard crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Non-derogable invariant: RoH must never exceed 0.3
pub const ROH_MAX: f32 = 0.3;

/// Non-derogable invariant: ROD HardStop threshold
pub const ROD_HARDSTOP: f32 = 1.0;
