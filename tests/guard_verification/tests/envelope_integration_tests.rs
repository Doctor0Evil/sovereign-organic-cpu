//! Biophysical Envelope Integration Tests
//!
//! End-to-end tests for complete biophysical envelope verification.

use organic_cpu_guard::{GuardContext, NeurorightsGuard, EvidenceBundle};
use biophysical_envelopes::{
    BiophysicalEnvelope, RoHCalculator, RoDCalculator, 
    LifeforceBand, LifeforceEnvelope, CytokineThresholds,
    EcoImpactScore, CEIM, NanoKarma,
};
use did_sovereignty::SovereignIdentity;
use neurorights_kernel::{InvariantSet, PolicyShard};

#[test]
fn test_complete_envelope_verification() {
    // Create biophysical envelope
    let mut roh_calc = RoHCalculator::new();
    roh_calc.add_eeg_load(0.2)
        .add_hrv_score(0.85)
        .add_thermal_delta(0.15)
        .add_duty_cycle(0.25)
        .add_cytokine_load(3.0, 1.5);
    
    let roh = roh_calc.build().unwrap();
    
    let mut rod_calc = RoDCalculator::new();
    rod_calc.add_pain_signal(0.1);
    let rod = rod_calc.build().unwrap();
    
    let cytokines = CytokineThresholds::new(3.0, 1.5, 5.0);
    let lifeforce = LifeforceEnvelope::new(cytokines, 0.85, 36.6, 0.9).unwrap();
    
    let ceim = CEIM::new(0.15, 0.1, 0.12);
    let nanokarma = NanoKarma::new(50, 0.01, 0.95);
    let eco = EcoImpactScore::new(ceim, nanokarma, 0.2);
    
    let envelope = BiophysicalEnvelope::new(
        roh,
        rod,
        lifeforce.band.clone(),
        eco,
    ).unwrap();
    
    // Verify envelope is safe
    assert!(envelope.is_safe());
    assert!(!envelope.is_warning());
    assert!(envelope.roh <= 0.3);
    assert!(envelope.rod < 1.0);
}

#[test]
fn test_guard_context_with_evidence_bundle() {
    let ctx = GuardContext::new(
        "did:bostrom:bostrom18...".to_string(),
        "organic_cpu_001".to_string(),
    );
    
    // Create evidence bundle with 10 tags
    let mut bundle = EvidenceBundle::empty();
    for i in 0..10 {
        bundle.add_anchor(&format!("0xtag{:02x}", i));
    }
    
    let ctx = ctx.with_evidence_bundle(bundle);
    
    // Context should validate with complete evidence bundle
    assert!(ctx.validate().is_ok());
}

#[test]
fn test_envelope_hardstop_conditions() {
    // Create envelope that triggers HardStop
    let cytokines = CytokineThresholds::new(15.0, 8.0, 30.0);
    let lifeforce = LifeforceEnvelope::new(cytokines, 0.3, 37.5, 0.2).unwrap();
    
    assert_eq!(lifeforce.band, LifeforceBand::HardStop);
    assert!(!lifeforce.is_safe());
    
    // High RoH should also trigger HardStop
    let mut roh_calc = RoHCalculator::new();
    roh_calc.add_eeg_load(0.9)
        .add_hrv_score(0.2)
        .add_thermal_delta(0.7)
        .add_duty_cycle(0.6)
        .add_cytokine_load(20.0, 10.0);
    
    assert!(roh_calc.build().is_err());
}

#[test]
fn test_eco_monotonicity_enforcement() {
    let ceim = CEIM::new(0.2, 0.15, 0.18);
    let nanokarma = NanoKarma::new(100, 0.02, 0.9);
    
    // Previous score is higher, so this should pass
    let eco = EcoImpactScore::new(ceim, nanokarma, 0.25);
    assert!(eco.verify_monotonicity().is_ok());
    assert!(eco.get_delta() <= 0.0);
    
    // Previous score is lower, so this should fail
    let eco_violation = EcoImpactScore::new(ceim, nanokarma, 0.1);
    assert!(eco_violation.verify_monotonicity().is_err());
    assert!(eco_violation.get_delta() > 0.0);
}

#[test]
fn test_lifeforce_operation_priority() {
    // Baseline allows all operations
    assert!(LifeforceBand::Baseline.allows_operation(1));
    assert!(LifeforceBand::Baseline.allows_operation(5));
    assert!(LifeforceBand::Baseline.allows_operation(10));
    
    // SoftWarn blocks low-priority operations
    assert!(!LifeforceBand::SoftWarn.allows_operation(3));
    assert!(LifeforceBand::SoftWarn.allows_operation(7));
    assert!(LifeforceBand::SoftWarn.allows_operation(10));
    
    // HardStop blocks almost everything
    assert!(!LifeforceBand::HardStop.allows_operation(5));
    assert!(LifeforceBand::HardStop.allows_operation(10));
}
