//! Policy Shards and Legal Regime Handling
//!
//! Implements "strictest-wins" logic for overlapping legal regimes.
//! Policies are compiled into ALN shards and enforced by the kernel.

use alloc::string::String;
use alloc::vec::Vec;
use crate::invariants::{NeurorightType, InvariantSet};
use crate::error::{KernelError, KernelResult};

/// Legal regimes supported by the kernel
#[derive(Clone, Debug, PartialEq)]
pub enum LegalRegime {
    CaliforniaSB1223,
    ColoradoHB241058,
    EUAIArticle5,
    CustomDIDBound,
}

impl LegalRegime {
    pub fn protectiveness_score(&self) -> u8 {
        // Higher score = more protective of neurorights
        match self {
            LegalRegime::CaliforniaSB1223 => 8,
            LegalRegime::ColoradoHB241058 => 9,
            LegalRegime::EUAIArticle5 => 7,
            LegalRegime::CustomDIDBound => 10, // DID-bound is most protective
        }
    }
}

/// Policy shard containing neurorights clauses
#[derive(Clone, Debug)]
pub struct PolicyShard {
    pub id: String,
    pub regime: LegalRegime,
    pub protected_rights: Vec<NeurorightType>,
    pub allows_downgrade: bool,
    pub requires_evolve_token: bool,
}

impl PolicyShard {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            regime: LegalRegime::CustomDIDBound,
            protected_rights: Vec::new(),
            allows_downgrade: false,
            requires_evolve_token: true,
        }
    }
    
    pub fn add_right_protection(&mut self, right: NeurorightType) {
        if !self.protected_rights.contains(&right) {
            self.protected_rights.push(right);
        }
    }
    
    pub fn protects_right(&self, right: &NeurorightType) -> bool {
        self.protected_rights.contains(right)
    }
    
    pub fn merge_strictest_wins(&mut self, other: &PolicyShard) {
        // Always adopt the more protective regime
        if other.regime.protectiveness_score() > self.regime.protectiveness_score() {
            self.regime = other.regime.clone();
        }
        
        // Union of protected rights
        for right in &other.protected_rights {
            self.add_right_protection(right.clone());
        }
        
        // Never allow downgrade if either policy forbids it
        if !other.allows_downgrade {
            self.allows_downgrade = false;
        }
    }
}

/// Strictest-wins policy manager
#[derive(Clone, Debug)]
pub struct StrictestWins {
    pub active_policy: PolicyShard,
    pub history: Vec<PolicyShard>,
}

impl StrictestWins {
    pub fn new(initial_policy: PolicyShard) -> Self {
        Self {
            active_policy: initial_policy,
            history: Vec::new(),
        }
    }
    
    pub fn apply_update(&mut self, new_policy: PolicyShard) -> KernelResult<()> {
        // Verify new policy is not less protective
        if new_policy.protected_rights.len() < self.active_policy.protected_rights.len() {
            return Err(KernelError::PolicyDowngradeAttempted);
        }
        
        self.history.push(self.active_policy.clone());
        self.active_policy.merge_strictest_wins(&new_policy);
        
        Ok(())
    }
    
    pub fn get_active_regime(&self) -> &LegalRegime {
        &self.active_policy.regime
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_strictest_wins_merge() {
        let mut policy1 = PolicyShard::new("policy1");
        policy1.regime = LegalRegime::CaliforniaSB1223;
        policy1.add_right_protection(NeurorightType::CognitiveLiberty);
        
        let mut policy2 = PolicyShard::new("policy2");
        policy2.regime = LegalRegime::ColoradoHB241058;
        policy2.add_right_protection(NeurorightType::MentalPrivacy);
        
        policy1.merge_strictest_wins(&policy2);
        
        assert_eq!(policy1.regime, LegalRegime::ColoradoHB241058);
        assert!(policy1.protects_right(&NeurorightType::CognitiveLiberty));
        assert!(policy1.protects_right(&NeurorightType::MentalPrivacy));
    }
    
    #[test]
    fn test_policy_downgrade_blocked() {
        let mut manager = StrictestWins::new(PolicyShard::new("initial"));
        manager.active_policy.add_right_protection(NeurorightType::CognitiveLiberty);
        
        let mut downgrade = PolicyShard::new("downgrade");
        // No rights added
        
        assert!(manager.apply_update(downgrade).is_err());
    }
}
