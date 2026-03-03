//! EcoImpactScore and Environmental Monotonicity
//!
//! Tracks environmental impact of cybernetic operations, ensuring
//! eco-monotonicity (non-regressive impact) across all evolution.

use alloc::string::String;
use crate::error::{EnvelopeError, EnvelopeResult};

/// Cybernetic Efficacy Index Metric
#[derive(Clone, Debug)]
pub struct CEIM {
    pub energy_consumption: f32,
    pub carbon_footprint: f32,
    pub resource_utilization: f32,
}

impl CEIM {
    pub fn new(energy: f32, carbon: f32, resource: f32) -> Self {
        Self {
            energy_consumption: energy,
            carbon_footprint: carbon,
            resource_utilization: resource,
        }
    }
    
    pub fn aggregate_score(&self) -> f32 {
        (self.energy_consumption + self.carbon_footprint + self.resource_utilization) / 3.0
    }
}

/// NanoKarma tracker for nanoswarm environmental impact
#[derive(Clone, Debug)]
pub struct NanoKarma {
    pub swarm_size: u32,
    pub energy_per_unit: f32,
    pub biodegradability_score: f32,
}

impl NanoKarma {
    pub fn new(swarm_size: u32, energy_per_unit: f32, biodegradability: f32) -> Self {
        Self {
            swarm_size,
            energy_per_unit,
            biodegradability_score: biodegradability,
        }
    }
    
    pub fn impact_score(&self) -> f32 {
        let raw_impact = (self.swarm_size as f32 * self.energy_per_unit) / 1000.0;
        raw_impact * (1.0 - self.biodegradability_score)
    }
}

/// EcoImpactScore with monotonicity verification
#[derive(Clone, Debug)]
pub struct EcoImpactScore {
    pub ceim: CEIM,
    pub nanokarma: NanoKarma,
    pub previous_score: f32,
    pub current_score: f32,
}

impl EcoImpactScore {
    pub fn new(ceim: CEIM, nanokarma: NanoKarma, previous_score: f32) -> Self {
        let current_score = (ceim.aggregate_score() + nanokarma.impact_score()) / 2.0;
        
        Self {
            ceim,
            nanokarma,
            previous_score,
            current_score,
        }
    }
    
    /// Verify eco-monotonicity (non-regressive impact)
    pub fn verify_monotonicity(&self) -> EnvelopeResult<bool> {
        if self.current_score > self.previous_score {
            return Err(EnvelopeError::EcoMonotonicityViolation {
                delta: self.current_score - self.previous_score,
            });
        }
        Ok(true)
    }
    
    pub fn get_delta(&self) -> f32 {
        self.current_score - self.previous_score
    }
    
    pub fn to_hex_anchor(&self) -> String {
        format!("0xeco{:08x}", (self.current_score * 1000.0) as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_eco_monotonicity_preserved() {
        let ceim = CEIM::new(0.2, 0.1, 0.15);
        let nanokarma = NanoKarma::new(100, 0.01, 0.9);
        
        let eco = EcoImpactScore::new(ceim, nanokarma, 0.2);
        
        assert!(eco.verify_monotonicity().is_ok());
        assert!(eco.get_delta() <= 0.0);
    }
    
    #[test]
    fn test_eco_monotonicity_violated() {
        let ceim = CEIM::new(0.5, 0.4, 0.45);
        let nanokarma = NanoKarma::new(500, 0.05, 0.3);
        
        let eco = EcoImpactScore::new(ceim, nanokarma, 0.1);
        
        assert!(eco.verify_monotonicity().is_err());
        assert!(eco.get_delta() > 0.0);
    }
}
