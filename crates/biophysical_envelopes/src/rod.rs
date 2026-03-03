//! Risk of Danger (ROD) Calculation
//!
//! ROD is an orthogonal danger metric (0.0 to 1.0) that accumulates pain debt
//! and neurorights budget consumption over time. ROD = 1.0 triggers HardStop.
//!
//! Unlike RoH (instantaneous), ROD is cumulative and tracks overdue harm.

use alloc::string::String;
use crate::error::{EnvelopeError, EnvelopeResult};

/// Pain debt accumulator
#[derive(Clone, Debug)]
pub struct PainDebt {
    pub current: f32,
    pub decay_rate: f32,
    pub threshold: f32,
}

impl PainDebt {
    pub fn new() -> Self {
        Self {
            current: 0.0,
            decay_rate: 0.1,
            threshold: 0.5,
        }
    }
    
    pub fn add(&mut self, pain_signal: f32) {
        self.current = (self.current + pain_signal).min(1.0);
    }
    
    pub fn decay(&mut self, time_delta: f32) {
        self.current = (self.current - self.decay_rate * time_delta).max(0.0);
    }
    
    pub fn contribution_to_rod(&self) -> f32 {
        self.current / self.threshold
    }
}

impl Default for PainDebt {
    fn default() -> Self {
        Self::new()
    }
}

/// Neurorights budget tracker
#[derive(Clone, Debug)]
pub struct NeurorightsBudget {
    pub cognitive_liberty_remaining: f32,
    pub mental_privacy_remaining: f32,
    pub augmentation_continuity_remaining: f32,
    pub project_continuity_remaining: f32,
}

impl NeurorightsBudget {
    pub fn new() -> Self {
        Self {
            cognitive_liberty_remaining: 1.0,
            mental_privacy_remaining: 1.0,
            augmentation_continuity_remaining: 1.0,
            project_continuity_remaining: 1.0,
        }
    }
    
    pub fn consume(&mut self, right: &str, amount: f32) -> EnvelopeResult<()> {
        let remaining = match right {
            "cognitive_liberty" => &mut self.cognitive_liberty_remaining,
            "mental_privacy" => &mut self.mental_privacy_remaining,
            "augmentation_continuity" => &mut self.augmentation_continuity_remaining,
            "project_continuity" => &mut self.project_continuity_remaining,
            _ => return Err(EnvelopeError::NeurorightsViolation {
                clause: right.to_string(),
                details: "Unknown right".to_string(),
            }),
        };
        
        *remaining = (*remaining - amount).max(0.0);
        Ok(())
    }
    
    pub fn average_remaining(&self) -> f32 {
        (self.cognitive_liberty_remaining
            + self.mental_privacy_remaining
            + self.augmentation_continuity_remaining
            + self.project_continuity_remaining) / 4.0
    }
    
    pub fn contribution_to_rod(&self) -> f32 {
        1.0 - self.average_remaining()
    }
}

impl Default for NeurorightsBudget {
    fn default() -> Self {
        Self::new()
    }
}

/// ROD Calculator with cumulative tracking
#[derive(Clone, Debug)]
pub struct RoDCalculator {
    pub pain_debt: PainDebt,
    pub neurorights_budget: NeurorightsBudget,
    pub historical_max: f32,
    pub timestamp: u64,
}

impl RoDCalculator {
    pub fn new() -> Self {
        Self {
            pain_debt: PainDebt::new(),
            neurorights_budget: NeurorightsBudget::new(),
            historical_max: 0.0,
            timestamp: 0,
        }
    }
    
    pub fn add_pain_signal(&mut self, pain: f32) -> &mut Self {
        self.pain_debt.add(pain);
        self
    }
    
    pub fn consume_neuroright(&mut self, right: &str, amount: f32) -> EnvelopeResult<&mut Self> {
        self.neurorights_budget.consume(right, amount)?;
        Ok(self)
    }
    
    pub fn calculate_rod(&self) -> f32 {
        let pain_contribution = self.pain_debt.contribution_to_rod() * 0.5;
        let rights_contribution = self.neurorights_budget.contribution_to_rod() * 0.5;
        
        let raw_rod = (pain_contribution + rights_contribution).min(1.0);
        raw_rod.max(self.historical_max)
    }
    
    pub fn check_hardstop(&self) -> bool {
        self.calculate_rod() >= 1.0
    }
    
    pub fn build(&self) -> EnvelopeResult<f32> {
        let rod = self.calculate_rod();
        
        if rod >= 1.0 {
            return Err(EnvelopeError::RodHardStop { current: rod });
        }
        
        Ok(rod)
    }
    
    pub fn update_historical_max(&mut self) {
        let current = self.calculate_rod();
        if current > self.historical_max {
            self.historical_max = current;
        }
    }
}

impl Default for RoDCalculator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rod_accumulation() {
        let mut calc = RoDCalculator::new();
        calc.add_pain_signal(0.3);
        
        let rod = calc.build();
        assert!(rod.is_ok());
        assert!(rod.unwrap() < 1.0);
    }
    
    #[test]
    fn test_rod_hardstop() {
        let mut calc = RoDCalculator::new();
        calc.add_pain_signal(0.9);
        calc.consume_neuroright("cognitive_liberty", 0.8).unwrap();
        
        let rod = calc.build();
        assert!(rod.is_err());
    }
    
    #[test]
    fn test_neurorights_budget() {
        let mut budget = NeurorightsBudget::new();
        budget.consume("cognitive_liberty", 0.3).unwrap();
        
        assert_eq!(budget.cognitive_liberty_remaining, 0.7);
    }
}
