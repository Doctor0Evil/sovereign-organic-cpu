//! Neurorights Kernel Core
//!
//! This crate compiles neurorights doctrine (cognitive liberty, mental privacy,
//! augmentation continuity, project continuity) into non-derogable invariants
//! enforced at the kernel level.
//!
//! # Security Model
//!
//! - **Strictest-Wins**: When multiple legal regimes apply, the most protective
//!   profile is automatically selected (e.g., SB 1223 > HB 24-1058).
//! - **Non-Derogable**: Invariants cannot be removed by updates, forks, or modifications.
//! - **Proposal-Only**: External entities can only propose changes; the kernel vetoes violations.
//!
//! # Example
//!
//! ```rust
//! use neurorights_kernel::{NeurorightsKernel, InvariantSet};
//!
//! let kernel = NeurorightsKernel::load_from_did(did)?;
//! let invariants = InvariantSet::default();
//!
//! if kernel.verify_invariants(&invariants)? {
//!     kernel.approve_upgrade(proposal)?;
//! } else {
//!     kernel.log_constitutional_violation(proposal)?;
//! }
//! ```

#![no_std]
#![cfg_attr(feature = "formal-verification", feature(custom_attribute))]
#![warn(missing_docs)]
#![warn(clippy::all)]
#![forbid(unsafe_code)]

extern crate alloc;

pub mod invariants;
pub mod policy;
pub mod audit;
pub mod error;

pub use invariants::{InvariantSet, NeurorightType, InvariantViolation};
pub use policy::{PolicyShard, LegalRegime, StrictestWins};
pub use audit::{ConstitutionalLog, ViolationRecord};
pub use error::{KernelError, KernelResult};

/// Core kernel trait for neurorights enforcement
pub trait NeurorightsEnforcer: Send + Sync {
    /// Verify that a proposal satisfies all non-derogable invariants
    fn verify_invariants(&self, proposal: &PolicyShard) -> KernelResult<bool>;
    
    /// Check if a specific neuroright is protected under current regime
    fn is_right_protected(&self, right: NeurorightType) -> bool;
    
    /// Log a constitutional violation (immutable)
    fn log_violation(&self, record: ViolationRecord) -> KernelResult<()>;
    
    /// Approve upgrade only if invariants are maintained or tightened
    fn approve_evolution(&self, proposal: &PolicyShard) -> KernelResult<()>;
}

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Non-derogable invariant: Cognitive Liberty
pub const INVARIANT_COGNITIVE_LIBERTY: &str = "no_nonconsensual_modulation";

/// Non-derogable invariant: Mental Privacy
pub const INVARIANT_MENTAL_PRIVACY: &str = "no_raw_neural_export";

/// Non-derogable invariant: Augmentation Continuity
pub const INVARIANT_AUGMENTATION_CONTINUITY: &str = "no_guard_removal";

/// Non-derogable invariant: Project Continuity
pub const INVARIANT_PROJECT_CONTINUITY: &str = "no_downgrade_without_consent";
