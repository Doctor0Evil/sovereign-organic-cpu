//! Biophysical Envelopes Core
//!
//! This crate implements quantitative, mathematical representations of the host's
//! physiological state, transforming abstract safety concepts into concrete,
//! measurable thresholds that govern system behavior.
//!
//! # Core Metrics
//!
//! | Metric | Constraint | Purpose |
//! |--------|------------|---------|
//! | RoH | ≤ 0.3 | Universal hard ceiling on admissible harm |
//! | ROD | < 1.0 | Cumulative strain guardrail (HardStop at 1.0) |
//! | LifeforceBand | Baseline/Warn/Stop | Health envelope gating |
//! | EcoImpactScore | ≤ 0.0 delta | Non-regressive environmental impact |
//!
//! # Example
//!
//! ```rust
//! use biophysical_envelopes::{BiophysicalEnvelope, RoHCalculator, LifeforceBand};
//!
//! let mut calc = RoHCalculator::new();
//! calc.add_eeg_load(0.4);
//! calc.add_hrv_score(0.7);
//! calc.add_thermal_delta(0.2);
//!
//! let envelope = calc.build()?;
//! assert!(envelope.roh <= 0.3);
//! assert_eq!(envelope.lifeforce_band, LifeforceBand::Baseline);
//! ```

#![no_std]
#![cfg_attr(feature = "formal-verification", feature(custom_attribute))]
#![warn(missing_docs)]
#![warn(clippy::all)]
#![forbid(unsafe_code)]

extern crate alloc;

pub mod roh;
pub mod rod;
pub mod lifeforce;
pub mod eco_impact;
pub mod telemetry;
pub mod error;

pub use roh::{RoHCalculator, RoHComponent};
pub use rod::{RoDCalculator, PainDebt, NeurorightsBudget};
pub use lifeforce::{LifeforceBand, LifeforceEnvelope, CytokineThresholds};
pub use eco_impact::{EcoImpactScore, CEIM, NanoKarma};
pub use telemetry::{BiophysicalTelemetry, EEGSnapshot, HRVSnapshot, ThermalSnapshot};
pub use error::{EnvelopeError, EnvelopeResult};

/// Core biophysical envelope structure
#[derive(Clone, Debug)]
pub struct BiophysicalEnvelope {
    pub roh: f32,
    pub rod: f32,
    pub lifeforce_band: LifeforceBand,
    pub eco_impact: EcoImpactScore,
    pub timestamp: u64,
    pub session_id: alloc::string::String,
}

impl BiophysicalEnvelope {
    pub fn new(
        roh: f32,
        rod: f32,
        lifeforce_band: LifeforceBand,
        eco_impact: EcoImpactScore,
    ) -> EnvelopeResult<Self> {
        if roh > 0.3 {
            return Err(EnvelopeError::RoHThresholdExceeded { current: roh });
        }
        if rod >= 1.0 {
            return Err(EnvelopeError::RodHardStop { current: rod });
        }
        
        Ok(Self {
            roh,
            rod,
            lifeforce_band,
            eco_impact,
            timestamp: 0,
            session_id: alloc::string::String::new(),
        })
    }
    
    pub fn is_safe(&self) -> bool {
        self.roh <= 0.3 && self.rod < 1.0 && self.lifeforce_band != LifeforceBand::HardStop
    }
    
    pub fn is_warning(&self) -> bool {
        self.lifeforce_band == LifeforceBand::SoftWarn || self.roh > 0.2
    }
    
    pub fn to_hex_anchor(&self) -> alloc::string::String {
        format!("0xenv{:08x}", (self.roh * 1000.0) as u32)
    }
}

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Non-derogable invariant: RoH must never exceed 0.3
pub const ROH_MAX: f32 = 0.3;

/// Non-derogable invariant: ROD HardStop threshold
pub const ROD_HARDSTOP: f32 = 1.0;

/// Evidence bundle hex anchors for biophysical envelopes
pub mod evidence_anchors {
    pub const BRAIN_POWER: &str = "0xpow20e7";
    pub const THERMAL_SAFETY: &str = "0xtherm3c";
    pub const CYTOKINE_THRESHOLDS: &str = "0xcyt0a9";
    pub const COGNITIVE_LOAD: &str = "0xcload7f";
    pub const ML_DUTY_CYCLE: &str = "0xlyaduty";
    pub const ROH_SCHEDULER: &str = "0xroh025c";
    pub const ECO_MONOTONICITY: &str = "0xeco9b2";
}
