//! Neurorights Guard Verification Tests
//!
//! Property-based tests for neurorights invariant enforcement.

use proptest::prelude::*;
use neurorights_kernel::{NeurorightsKernel, InvariantSet, NeurorightType, PolicyShard};
use did_sovereignty::{SovereignIdentity, EvolveToken};

proptest! {
    #[test]
    fn test_invariant_set_always_locked(
        seed in any::<u64>(),
    ) {
        let mut set = InvariantSet::new();
        
        // Invariant set should always be locked by default
        prop_assert!(set.is_locked());
        
        // Cannot unlock without valid signature
        let result = set.lock("");
        prop_assert!(result.is_err());
    }
    
    #[test]
    fn test_policy_never_downgrades(
        initial_rights_count in 1usize..4,
        new_rights_count in 1usize..4,
    ) {
        let mut initial_policy = PolicyShard::new("initial");
        for i in 0..initial_rights_count {
            match i % 4 {
                0 => initial_policy.add_right_protection(NeurorightType::CognitiveLiberty),
                1 => initial_policy.add_right_protection(NeurorightType::MentalPrivacy),
                2 => initial_policy.add_right_protection(NeurorightType::AugmentationContinuity),
                _ => initial_policy.add_right_protection(NeurorightType::ProjectContinuity),
            }
        }
        
        let mut new_policy = PolicyShard::new("new");
        for i in 0..new_rights_count {
            match i % 4 {
                0 => new_policy.add_right_protection(NeurorightType::CognitiveLiberty),
                1 => new_policy.add_right_protection(NeurorightType::MentalPrivacy),
                2 => new_policy.add_right_protection(NeurorightType::AugmentationContinuity),
                _ => new_policy.add_right_protection(NeurorightType::ProjectContinuity),
            }
        }
        
        // Merge should adopt strictest-wins
        initial_policy.merge_strictest_wins(&new_policy);
        
        // Rights count should never decrease
        prop_assert!(initial_policy.protected_rights.len() >= initial_rights_count);
    }
    
    #[test]
    fn test_evolve_token_always_bound_to_did(
        did_suffix in "[a-z0-9]{10,20}",
        upgrade_hash in "[a-f0-9]{64}",
    ) {
        let did = format!("did:bostrom:{}", did_suffix);
        let identity = SovereignIdentity::new(did.clone(), "cpu_001".to_string());
        
        if let Ok(mut identity) = identity {
            let _ = identity.bind_to_did("public_key".to_string());
            let _ = identity.verify("boot_hash");
            
            if identity.is_sovereign() {
                let token = EvolveToken::mint_for_upgrade(&identity, upgrade_hash.to_string());
                
                if let Ok(token) = token {
                    // Token must be bound to the same DID
                    prop_assert!(token.is_bound_to_did(&did));
                }
            }
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    
    #[test]
    fn test_invariant_set_default_rights() {
        let set = InvariantSet::default();
        assert_eq!(set.rights.len(), 4);
        assert!(set.rights.contains(&NeurorightType::CognitiveLiberty));
        assert!(set.rights.contains(&NeurorightType::MentalPrivacy));
        assert!(set.rights.contains(&NeurorightType::AugmentationContinuity));
        assert!(set.rights.contains(&NeurorightType::ProjectContinuity));
    }
    
    #[test]
    fn test_policy_strictest_wins_regime() {
        let mut policy1 = PolicyShard::new("policy1");
        policy1.regime = neurorights_kernel::LegalRegime::CaliforniaSB1223;
        
        let mut policy2 = PolicyShard::new("policy2");
        policy2.regime = neurorights_kernel::LegalRegime::ColoradoHB241058;
        
        policy1.merge_strictest_wins(&policy2);
        
        // Colorado HB 24-1058 is more protective (score 9 vs 8)
        assert_eq!(policy1.regime, neurorights_kernel::LegalRegime::ColoradoHB241058);
    }
    
    #[test]
    fn test_neuroright_type_conversion() {
        let right = NeurorightType::CognitiveLiberty;
        let invariant_str = right.to_invariant_string();
        assert_eq!(invariant_str, "no_nonconsensual_modulation");
        
        let recovered = NeurorightType::from_invariant_string(invariant_str);
        assert_eq!(recovered, Some(NeurorightType::CognitiveLiberty));
    }
}
