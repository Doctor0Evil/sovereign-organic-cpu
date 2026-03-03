//! DID Sovereignty Core
//!
//! This crate implements DID-bound boot chains, EVOLVE token gating, and
//! sovereign identity management. The DID serves as the root of trust for
//! all operations, ensuring that identity cannot be separated from enforcement.
//!
//! # Security Model
//!
//! - **Boot Identity**: DID pairs with OrganicCPU at first boot; all root keys derive from this
//! - **EVOLVE Tokens**: Non-transferable, DID-bound tokens gate all upgrades
//! - **Non-Derogable**: Identity cannot be downgraded or excluded without cryptographic evidence
//!
//! # Example
//!
//! ```rust
//! use did_sovereignty::{SovereignIdentity, EvolveToken, BootChain};
//!
//! let identity = SovereignIdentity::bind_to_did(did, organic_cpu_id)?;
//! let boot = BootChain::verify(&identity)?;
//! let token = EvolveToken::mint_for_upgrade(&identity, upgrade_hash)?;
//!
//! if token.validate()? {
//!     identity.approve_evolution(upgrade)?;
//! }
//! ```

#![no_std]
#![cfg_attr(feature = "formal-verification", feature(custom_attribute))]
#![warn(missing_docs)]
#![warn(clippy::all)]
#![forbid(unsafe_code)]

extern crate alloc;

pub mod identity;
pub mod boot_chain;
pub mod evolve_token;
pub mod error;

pub use identity::{SovereignIdentity, IdentityState, DIDBinding};
pub use boot_chain::{BootChain, BootHash, SecureBootVerifier};
pub use evolve_token::{EvolveToken, TokenState, UpgradeProposal};
pub use error::{SovereigntyError, SovereigntyResult};

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// DID prefix for Bostrom chain
pub const BOSTROM_DID_PREFIX: &str = "did:bostrom:";

/// Non-derogable invariant: Identity cannot be separated from enforcement
pub const INVARIANT_IDENTITY_ENFORCEMENT: &str = "no_identity_without_enforcement";

/// Evidence bundle hex anchors for DID sovereignty
pub mod evidence_anchors {
    pub const DID_BINDING: &str = "0xdidbind01";
    pub const BOOT_CHAIN: &str = "0xbootchn02";
    pub const EVOLVE_TOKEN: &str = "0xevolve03";
    pub const IDENTITY_CONTINUITY: &str = "0xidentcont04";
}
