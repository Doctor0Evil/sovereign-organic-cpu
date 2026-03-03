//! EVOLVE Token System
//!
//! Non-transferable, DID-bound tokens that gate all system upgrades.
//! Tokens can only permit envelope-tightening or equal-safety upgrades.

use alloc::string::String;
use alloc::vec::Vec;
use crate::identity::SovereignIdentity;
use crate::error::{SovereigntyError, SovereigntyResult};
use crate::evidence_anchors;

/// Token state enumeration
#[derive(Clone, Debug, PartialEq)]
pub enum TokenState {
    /// Token minted, not yet used
    Active,
    /// Token consumed for upgrade
    Consumed,
    /// Token expired
    Expired,
    /// Token revoked (violation detected)
    Revoked,
}

/// Upgrade proposal structure
#[derive(Clone, Debug)]
pub struct UpgradeProposal {
    pub proposal_id: String,
    pub upgrade_hash: String,
    pub evidence_bundle: Vec<String>,
    pub safety_delta: f32, // Negative = safer, Positive = riskier
    pub proposer_did: String,
}

impl UpgradeProposal {
    pub fn new(proposal_id: String, upgrade_hash: String, proposer_did: String) -> Self {
        Self {
            proposal_id,
            upgrade_hash,
            evidence_bundle: Vec::new(),
            safety_delta: 0.0,
            proposer_did,
        }
    }
    
    pub fn add_evidence_tag(&mut self, tag: &str) {
        if !self.evidence_bundle.contains(&tag.to_string()) {
            self.evidence_bundle.push(tag.to_string());
        }
    }
    
    pub fn is_safety_improving(&self) -> bool {
        self.safety_delta <= 0.0
    }
    
    pub fn validate_evidence_completeness(&self) -> SovereigntyResult<()> {
        if self.evidence_bundle.len() < 10 {
            return Err(SovereigntyError::IncompleteEvidenceBundle {
                current: self.evidence_bundle.len(),
                required: 10,
            });
        }
        Ok(())
    }
}

/// EVOLVE Token
#[derive(Clone, Debug)]
pub struct EvolveToken {
    pub token_id: String,
    pub bound_did: String,
    pub state: TokenState,
    pub minted_at: u64,
    pub expires_at: u64,
    pub consumed_for_upgrade: Option<String>,
}

impl EvolveToken {
    pub fn mint_for_upgrade(identity: &SovereignIdentity, upgrade_hash: String) -> SovereigntyResult<Self> {
        if !identity.is_sovereign() {
            return Err(SovereigntyError::IdentityNotVerified);
        }
        
        Ok(Self {
            token_id: format!("evolve_{}", upgrade_hash),
            bound_did: identity.binding.did.clone(),
            state: TokenState::Active,
            minted_at: 0,
            expires_at: 0,
            consumed_for_upgrade: None,
        })
    }
    
    pub fn validate(&self) -> SovereigntyResult<bool> {
        match self.state {
            TokenState::Active => Ok(true),
            TokenState::Consumed => Err(SovereigntyError::TokenAlreadyConsumed),
            TokenState::Expired => Err(SovereigntyError::TokenExpired),
            TokenState::Revoked => Err(SovereigntyError::TokenRevoked),
        }
    }
    
    pub fn consume(&mut self, upgrade_hash: String) -> SovereigntyResult<()> {
        if self.state != TokenState::Active {
            return Err(SovereigntyError::TokenNotActive);
        }
        
        // Verify upgrade hash matches token
        if !self.token_id.contains(&upgrade_hash) {
            return Err(SovereigntyError::TokenUpgradeMismatch);
        }
        
        self.state = TokenState::Consumed;
        self.consumed_for_upgrade = Some(upgrade_hash);
        Ok(())
    }
    
    pub fn is_bound_to_did(&self, did: &str) -> bool {
        self.bound_did == did
    }
    
    pub fn to_hex_anchor(&self) -> String {
        format!("0xevolve{:08x}", self.token_id.len() as u32)
    }
}

/// EVOLVE Token manager
#[derive(Clone, Debug)]
pub struct EvolveTokenManager {
    pub active_tokens: Vec<EvolveToken>,
    pub consumed_tokens: Vec<EvolveToken>,
}

impl EvolveTokenManager {
    pub fn new() -> Self {
        Self {
            active_tokens: Vec::new(),
            consumed_tokens: Vec::new(),
        }
    }
    
    pub fn mint_token(&mut self, identity: &SovereignIdentity, upgrade_hash: String) -> SovereigntyResult<EvolveToken> {
        let token = EvolveToken::mint_for_upgrade(identity, upgrade_hash)?;
        self.active_tokens.push(token.clone());
        Ok(token)
    }
    
    pub fn consume_token(&mut self, token_id: &str, upgrade_hash: String) -> SovereigntyResult<()> {
        // Find and consume token
        for token in &mut self.active_tokens {
            if token.token_id == token_id {
                token.consume(upgrade_hash)?;
                self.consumed_tokens.push(token.clone());
                return Ok(());
            }
        }
        Err(SovereigntyError::TokenNotFound)
    }
    
    pub fn get_active_token_count(&self) -> usize {
        self.active_tokens.len()
    }
}

impl Default for EvolveTokenManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_token_minting() {
        let identity = SovereignIdentity::new(
            "did:bostrom:bostrom18...".to_string(),
            "organic_cpu_001".to_string(),
        ).unwrap();
        
        // Cannot mint until identity is verified
        assert!(EvolveToken::mint_for_upgrade(&identity, "upgrade_hash".to_string()).is_err());
    }
    
    #[test]
    fn test_upgrade_proposal_validation() {
        let mut proposal = UpgradeProposal::new(
            "proposal_001".to_string(),
            "upgrade_hash".to_string(),
            "did:bostrom:proposer".to_string(),
        );
        
        // Should fail without 10 evidence tags
        assert!(proposal.validate_evidence_completeness().is_err());
        
        // Add 10 tags
        for i in 0..10 {
            proposal.add_evidence_tag(&format!("0xtag{:02x}", i));
        }
        
        assert!(proposal.validate_evidence_completeness().is_ok());
    }
}
