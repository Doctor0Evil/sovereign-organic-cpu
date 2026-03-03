//! reality.os Integration Layer
//!
//! This crate provides kernel-level integration for OrganicCPU guards,
//! neurorights enforcement, and biophysical envelope gating within reality.os.
//!
//! # Security Model
//!
//! - **Kernel-Enforced**: Guards run as kernel services, not userland daemons
//! - **Syscall Gating**: All neural device access wrapped in guard checks
//! - **Device Classification**: INNER vs OUTER domain enforced at kernel level
//! - **Non-Derogable**: Neurorights compiled into kernel invariants
//!
//! # Example
//!
//! ```rust
//! use reality_os_integration::{KernelGuard, SyscallWrapper, DeviceClassifier};
//!
//! let kernel_guard = KernelGuard::load()?;
//! let wrapper = SyscallWrapper::new(kernel_guard);
//!
//! // All syscalls to INNER devices now automatically vetted
//! wrapper.wrap_open("/dev/bci_inner_001")?;
//! ```

#![no_std]
#![cfg_attr(feature = "kernel-module", feature(no_core))]
#![warn(missing_docs)]
#![warn(clippy::all)]
#![forbid(unsafe_code)]

extern crate alloc;

pub mod kernel_guard;
pub mod syscall_wrapper;
pub mod device_classifier;
pub mod audit_log;
pub mod error;

pub use kernel_guard::{KernelGuard, GuardService};
pub use syscall_wrapper::{SyscallWrapper, WrappedSyscall};
pub use device_classifier::{DeviceClassifier, DomainLabel};
pub use audit_log::{KernelAuditLog, AuditEntry};
pub use error::{IntegrationError, IntegrationResult};

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// reality.os kernel version compatibility
pub const REALITY_OS_KERNEL_VERSION: &str = "0.3.0";

/// Non-derogable invariant: Kernel cannot load modules without guard approval
pub const INVARIANT_KERNEL_MODULE_LOADING: &str = "no_module_without_guard_approval";

/// Evidence bundle hex anchors for reality.os integration
pub mod evidence_anchors {
    pub const KERNEL_GUARD: &str = "0xkernelg01";
    pub const SYSCALL_GATE: &str = "0xsysgate02";
    pub const DEVICE_CLASS: &str = "0xdevclass03";
    pub const AUDIT_TRAIL: &str = "0xaudit04";
}
