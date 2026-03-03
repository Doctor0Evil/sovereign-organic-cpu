//! LifeforceBand Envelope
//!
//! LifeforceBand provides a broader health envelope calibrated from multiple
//! biophysical sources. Used as the central gating mechanism for all XR,
//! nanoswarm, and BCI loads.
//!
//! States: Baseline → SoftWarn → HardStop

use alloc::string::String;
use crate::error::{EnvelopeError, EnvelopeResult};

/// Lifeforce band states
#[derive(Clone, Debug, PartialEq)]
pub enum LifeforceBand {
    /// Normal operating state
    Baseline,
    /// Warning state - reduce load
    SoftWarn,
    /// Critical state - all non-essential operations blocked
    HardStop,
}

impl LifeforceBand {
    pub fn from_roh_rod(roh: f32, rod: f32) -> Self {
        if roh > 0.25 || rod > 0.7 {
            LifeforceBand::HardStop
        } else if roh > 0.15 || rod > 0.4 {
            LifeforceBand::SoftWarn
        } else {
            LifeforceBand::Baseline
        }
    }
    
    pub fn allows_operation(&self, operation_priority: u8) -> bool {
        match self {
            LifeforceBand::Baseline => true,
            LifeforceBand::SoftWarn => operation_priority >= 5,
            LifeforceBand::HardStop => operation_priority >= 10,
        }
    }
}

/// Cytokine threshold tracker
#[derive(Clone, Debug)]
pub struct CytokineThresholds {
    pub il6: f32,
    pub crp: f32,
    pub tnf_alpha: f32,
}

impl CytokineThresholds {
    pub fn new(il6: f32, crp: f32, tnf_alpha: f32) -> Self {
        Self { il6, crp, tnf_alpha }
    }
    
    pub fn is_elevated(&self) -> bool {
        self.il6 > 10.0 || self.crp > 5.0 || self.tnf_alpha > 20.0
    }
    
    pub fn contribution_to_lifeforce(&self) -> f32 {
        let il6_score = (self.il6 / 10.0).min(1.0);
        let crp_score = (self.crp / 5.0).min(1.0);
        let tnf_score = (self.tnf_alpha / 20.0).min(1.0);
        
        (il6_score + crp_score + tnf_score) / 3.0
    }
}

/// Complete LifeforceEnvelope
#[derive(Clone, Debug)]
pub struct LifeforceEnvelope {
    pub band: LifeforceBand,
    pub cytokines: CytokineThresholds,
    pub hrv_score: f32,
    pub thermal_baseline: f32,
    pub subjective_report: f32,
}

impl LifeforceEnvelope {
    pub fn new(
        cytokines: CytokineThresholds,
        hrv_score: f32,
        thermal_baseline: f32,
        subjective_report: f32,
    ) -> EnvelopeResult<Self> {
        let cytokine_load = cytokines.contribution_to_lifeforce();
        let combined_load = (cytokine_load + (1.0 - hrv_score) + (1.0 - subjective_report)) / 3.0;
        
        let band = if combined_load > 0.7 {
            LifeforceBand::HardStop
        } else if combined_load > 0.4 {
            LifeforceBand::SoftWarn
        } else {
            LifeforceBand::Baseline
        };
        
        Ok(Self {
            band,
            cytokines,
            hrv_score,
            thermal_baseline,
            subjective_report,
        })
    }
    
    pub fn is_safe(&self) -> bool {
        self.band != LifeforceBand::HardStop
    }
    
    pub fn is_warning(&self) -> bool {
        self.band == LifeforceBand::SoftWarn
    }
    
    pub fn to_hex_anchor(&self) -> String {
        match self.band {
            LifeforceBand::Baseline => "0xlfb_baseline".to_string(),
            LifeforceBand::SoftWarn => "0xlfb_warn".to_string(),
            LifeforceBand::HardStop => "0xlfb_stop".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lifeforce_baseline() {
        let cytokines = CytokineThresholds::new(3.0, 1.0, 5.0);
        let envelope = LifeforceEnvelope::new(cytokines, 0.8, 36.5, 0.9).unwrap();
        
        assert_eq!(envelope.band, LifeforceBand::Baseline);
        assert!(envelope.is_safe());
    }
    
    #[test]
    fn test_lifeforce_hardstop() {
        let cytokines = CytokineThresholds::new(15.0, 8.0, 30.0);
        let envelope = LifeforceEnvelope::new(cytokines, 0.3, 37.5, 0.2).unwrap();
        
        assert_eq!(envelope.band, LifeforceBand::HardStop);
        assert!(!envelope.is_safe());
    }
    
    #[test]
    fn test_operation_priority() {
        let band = LifeforceBand::SoftWarn;
        
        assert!(band.allows_operation(7));
        assert!(!band.allows_operation(3));
    }
}
