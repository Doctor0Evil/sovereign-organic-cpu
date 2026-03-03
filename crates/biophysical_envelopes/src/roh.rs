//! Risk of Harm (RoH) Calculation
//!
//! RoH is a scalar value (0.0 to 1.0) representing the instantaneous risk
//! of harm to the host from cybernetic operations. Must never exceed 0.3.
//!
//! Calculation components:
//! - EEG load (alpha/gamma ratios, theta-beta)
//! - HRV score (heart rate variability)
//! - Thermal delta (temperature change)
//! - Duty cycle (normalized active time)
//! - Cytokine levels (IL-6, CRP)

use alloc::vec::Vec;
use crate::error::{EnvelopeError, EnvelopeResult};
use crate::telemetry::{EEGSnapshot, HRVSnapshot, ThermalSnapshot};

/// Individual RoH component with weight
#[derive(Clone, Debug)]
pub struct RoHComponent {
    pub name: alloc::string::String,
    pub value: f32,
    pub weight: f32,
    pub threshold: f32,
}

impl RoHComponent {
    pub fn new(name: &str, value: f32, weight: f32, threshold: f32) -> Self {
        Self {
            name: name.to_string(),
            value,
            weight,
            threshold,
        }
    }
    
    pub fn normalized_contribution(&self) -> f32 {
        (self.value / self.threshold).min(1.0) * self.weight
    }
}

/// RoH Calculator implementing Lyapunov-style stability
#[derive(Clone, Debug)]
pub struct RoHCalculator {
    components: Vec<RoHComponent>,
    lyapunov_residual: f32,
    timestamp: u64,
}

impl RoHCalculator {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            lyapunov_residual: 0.0,
            timestamp: 0,
        }
    }
    
    /// Add EEG load component
    pub fn add_eeg_load(&mut self, load: f32) -> &mut Self {
        self.components.push(RoHComponent::new(
            "eeg_load",
            load,
            0.25,
            0.8,
        ));
        self
    }
    
    /// Add HRV score component
    pub fn add_hrv_score(&mut self, hrv: f32) -> &mut Self {
        self.components.push(RoHComponent::new(
            "hrv_score",
            1.0 - hrv,
            0.20,
            0.5,
        ));
        self
    }
    
    /// Add thermal delta component
    pub fn add_thermal_delta(&mut self, delta: f32) -> &mut Self {
        self.components.push(RoHComponent::new(
            "thermal_delta",
            delta,
            0.20,
            0.5,
        ));
        self
    }
    
    /// Add duty cycle component
    pub fn add_duty_cycle(&mut self, duty: f32) -> &mut Self {
        self.components.push(RoHComponent::new(
            "duty_cycle",
            duty,
            0.15,
            0.4,
        ));
        self
    }
    
    /// Add cytokine load component
    pub fn add_cytokine_load(&mut self, il6: f32, crp: f32) -> &mut Self {
        let cytokine_score = (il6 / 10.0 + crp / 5.0) / 2.0;
        self.components.push(RoHComponent::new(
            "cytokine_load",
            cytokine_score,
            0.20,
            1.0,
        ));
        self
    }
    
    /// Calculate Lyapunov residual for stability verification
    pub fn calculate_lyapunov_residual(&mut self, previous_roh: f32) -> f32 {
        let current_roh = self.calculate_raw();
        self.lyapunov_residual = (current_roh - previous_roh).max(0.0);
        self.lyapunov_residual
    }
    
    /// Calculate raw RoH value (before clamping)
    pub fn calculate_raw(&self) -> f32 {
        if self.components.is_empty() {
            return 0.0;
        }
        
        let weighted_sum: f32 = self.components
            .iter()
            .map(|c| c.normalized_contribution())
            .sum();
        
        let total_weight: f32 = self.components.iter().map(|c| c.weight).sum();
        
        if total_weight == 0.0 {
            return 0.0;
        }
        
        weighted_sum / total_weight
    }
    
    /// Build envelope with RoH ≤ 0.3 constraint
    pub fn build(&self) -> EnvelopeResult<f32> {
        let raw_roh = self.calculate_raw();
        
        if raw_roh > 0.3 {
            return Err(EnvelopeError::RoHThresholdExceeded { current: raw_roh });
        }
        
        Ok(raw_roh)
    }
    
    /// Get component breakdown for audit
    pub fn get_component_breakdown(&self) -> &Vec<RoHComponent> {
        &self.components
    }
}

impl Default for RoHCalculator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_roh_below_threshold() {
        let mut calc = RoHCalculator::new();
        calc.add_eeg_load(0.3)
            .add_hrv_score(0.8)
            .add_thermal_delta(0.2)
            .add_duty_cycle(0.25)
            .add_cytokine_load(3.0, 1.0);
        
        let roh = calc.build();
        assert!(roh.is_ok());
        assert!(roh.unwrap() <= 0.3);
    }
    
    #[test]
    fn test_roh_exceeds_threshold() {
        let mut calc = RoHCalculator::new();
        calc.add_eeg_load(0.9)
            .add_hrv_score(0.2)
            .add_thermal_delta(0.6)
            .add_duty_cycle(0.5)
            .add_cytokine_load(15.0, 8.0);
        
        let roh = calc.build();
        assert!(roh.is_err());
    }
    
    #[test]
    fn test_lyapunov_stability() {
        let mut calc = RoHCalculator::new();
        calc.add_eeg_load(0.3).add_hrv_score(0.8);
        
        let residual = calc.calculate_lyapunov_residual(0.25);
        assert!(residual >= 0.0);
    }
}
