//! ALN Protocol Stack Core
//!
//! This crate implements the ALN-governed networking stack where every message
//! contains metadata for OrganicCPU guard evaluation. It replaces legacy protocols
//! with a sovereign stack ensuring corridor-safe semantics.
//!
//! # Security Model
//!
//! - **Inner Domain**: BCI/EEG/nanoswarm channels (never exposed externally)
//! - **Outer Domain**: Corridor-safe scalars only (RoH, BCI bands, LifeforceBand)
//! - **Proposal-Only**: All external interactions are signed proposals
//! - **Legacy Shim**: ROS/BLE/CAN connect only through strictly typed gateways
//!
//! # Example
//!
//! ```rust
//! use aln_protocol::{ALNParticle, ChannelType, SovereignRouter};
//!
//! let particle = ALNParticle::new_proposal(did, operation)?;
//! let router = SovereignRouter::new(organic_cpu_id);
//!
//! if router.validate_corridor(&particle)? {
//!     router.route_outer(particle)?;
//! } else {
//!     router.reject_with_violation_log(particle)?;
//! }
//! ```

#![no_std]
#![cfg_attr(feature = "formal-verification", feature(custom_attribute))]
#![warn(missing_docs)]
#![warn(clippy::all)]
#![forbid(unsafe_code)]

extern crate alloc;

pub mod packet;
pub mod channel;
pub mod gateway;
pub mod router;
pub mod error;

pub use packet::{ALNParticle, ParticleType, EvidenceHeader};
pub use channel::{ChannelType, SovereignChannel, InnerDomain, OuterDomain};
pub use gateway::{LegacyShim, Ros2Shim, BleShim};
pub use router::{SovereignRouter, RoutingVerdict};
pub use error::{ProtocolError, ProtocolResult};

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Protocol identifier for ALN sovereign stack
pub const PROTOCOL_ID: &str = "/aln/sovereign/1.0.0";

/// Evidence bundle hex anchors for networking
pub mod evidence_anchors {
    pub const NETWORK_CORRIDOR: &str = "0xnetcorr01";
    pub const DID_ROUTING: &str = "0xdidroute02";
    pub const ENCRYPTION_LAYER: &str = "0xencrypt03";
}
