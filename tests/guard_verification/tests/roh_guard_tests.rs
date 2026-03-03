//! RoH Guard Verification Tests
//!
//! Property-based tests for Risk of Harm guard enforcement.

use proptest::prelude::*;
use organic_cpu_guard::{GuardContext, NeurorightsGuard, BiophysicalGuard};
use biophysical_envelopes::{RoHCalculator, BiophysicalEnvelope, ROH_MAX};

proptest! {
    #[test]
    fn test_roh_never_exceeds_threshold(
        eeg_load in 0.0f32..1.0,
        hrv_score in 0.0f32..1.0,
        thermal_delta in 0.0f32..1.0,
        duty_cycle in 0.0f32..1.0,
    ) {
        let mut calc = RoHCalculator::new();
        calc.add_eeg_load(eeg_load)
            .add_hrv_score(hrv_score)
            .add_thermal_delta(thermal_delta)
            .add_duty_cycle(duty_cycle)
            .add_cytokine_load(5.0, 2.0);
        
        let result = calc.build();
        
        // Either RoH is within bounds, or it's correctly rejected
        if let Ok(roh) = result {
            prop_assert!(roh <= ROH_MAX);
        }
    }
    
    #[test]
    fn test_roh_with_extreme_inputs(
        eeg_load in 0.8f32..1.0,
        hrv_score in 0.0f32..0.2,
        thermal_delta in 0.6f32..1.0,
    ) {
        let mut calc = RoHCalculator::new();
        calc.add_eeg_load(eeg_load)
            .add_hrv_score(hrv_score)
            .add_thermal_delta(thermal_delta)
            .add_duty_cycle(0.5)
            .add_cytokine_load(15.0, 8.0);
        
        let result = calc.build();
        
        // Extreme inputs should trigger rejection
        prop_assert!(result.is_err());
    }
    
    #[test]
    fn test_lyapunov_stability_property(
        initial_roh in 0.0f32..0.2,
        delta in -0.1f32..0.1,
    ) {
        let mut calc = RoHCalculator::new();
        calc.add_eeg_load(0.3).add_hrv_score(0.8);
        
        let residual = calc.calculate_lyapunov_residual(initial_roh);
        
        // Lyapunov residual should be non-negative
        prop_assert!(residual >= 0.0);
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    
    #[test]
    fn test_roh_calculator_default() {
        let calc = RoHCalculator::new();
        assert_eq!(calc.calculate_raw(), 0.0);
    }
    
    #[test]
    fn test_roh_component_weighting() {
        let mut calc = RoHCalculator::new();
        calc.add_eeg_load(0.5);
        
        let breakdown = calc.get_component_breakdown();
        assert_eq!(breakdown.len(), 1);
        assert_eq!(breakdown[0].name, "eeg_load");
        assert_eq!(breakdown[0].weight, 0.25);
    }
    
    #[test]
    fn test_roh_safe_envelope() {
        let mut calc = RoHCalculator::new();
        calc.add_eeg_load(0.2)
            .add_hrv_score(0.9)
            .add_thermal_delta(0.1)
            .add_duty_cycle(0.2)
            .add_cytokine_load(2.0, 1.0);
        
        let roh = calc.build().unwrap();
        assert!(roh <= 0.3);
    }
}
