//! Sovereign Identity Management
//!
//! Binds the augmented citizen's DID to their OrganicCPU instance,
//! ensuring identity cannot be separated from enforcement.

use alloc::string::String;
use alloc::vec::Vec;
use crate::error::{SovereigntyError, SovereigntyResult};
use crate::evidence_anchors;

/// DID binding structure
#[derive(Clone, Debug)]
pub struct DIDBinding {
    pub did: String,
    pub public_key: String,
    pub key_type: String,
    pub created_at: u64,
}

impl DIDBinding {
    pub fn new(did: String, public_key: String) -> Self {
        Self {
            did,
            public_key,
            key_type: "Ed25519".to_string(),
            created_at: 0,
        }
    }
    
    pub fn validate_format(&self) -> SovereigntyResult<()> {
        if !self.did.starts_with("did:bostrom:") {
            return Err(SovereigntyError::InvalidDIDFormat {
                did: self.did.clone(),
                expected_prefix: "did:bostrom:".to_string(),
            });
        }
        if self.public_key.is_empty() {
            return Err(SovereigntyError::MissingPublicKey);
        }
        Ok(())
    }
    
    pub fn to_hex_anchor(&self) -> String {
        format!("0xdid{:08x}", self.did.len() as u32)
    }
}

/// Identity state enumeration
#[derive(Clone, Debug, PartialEq)]
pub enum IdentityState {
    /// Fresh identity, not yet bound
    Unbound,
    /// DID bound to OrganicCPU
    Bound,
    /// Identity verified with boot chain
    Verified,
    /// Identity compromised (requires recovery)
    Compromised,
}

/// Complete sovereign identity
#[derive(Clone, Debug)]
pub struct SovereignIdentity {
    pub binding: DIDBinding,
    pub organic_cpu_id: String,
    pub state: IdentityState,
    pub evolution_history: Vec<String>,
    pub last_verification: u64,
}

impl SovereignIdentity {
    pub fn new(did: String, organic_cpu_id: String) -> SovereigntyResult<Self> {
        let binding = DIDBinding::new(did, String::new());
        binding.validate_format()?;
        
        Ok(Self {
            binding,
            organic_cpu_id,
            state: IdentityState::Unbound,
            evolution_history: Vec::new(),
            last_verification: 0,
        })
    }
    
    pub fn bind_to_did(&mut self, public_key: String) -> SovereigntyResult<()> {
        self.binding.public_key = public_key;
        self.binding.validate_format()?;
        self.state = IdentityState::Bound;
        Ok(())
    }
    
    pub fn verify(&mut self, boot_hash: &str) -> SovereigntyResult<()> {
        if self.state != IdentityState::Bound {
            return Err(SovereigntyError::IdentityNotBound);
        }
        
        // In real implementation, verify boot hash cryptographically
        if boot_hash.is_empty() {
            return Err(SovereigntyError::InvalidBootHash);
        }
        
        self.state = IdentityState::Verified;
        self.last_verification = 0; // Would be timestamp
        Ok(())
    }
    
    pub fn record_evolution(&mut self, upgrade_hash: String) -> SovereigntyResult<()> {
        if self.state != IdentityState::Verified {
            return Err(SovereigntyError::IdentityNotVerified);
        }
        
        self.evolution_history.push(upgrade_hash);
        Ok(())
    }
    
    pub fn is_sovereign(&self) -> bool {
        self.state == IdentityState::Verified
    }
    
    pub fn get_evolution_count(&self) -> usize {
        self.evolution_history.len()
    }
    
    pub fn to_hex_anchor(&self) -> String {
        format!("0xsov{:08x}", self.evolution_history.len() as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_identity_creation() {
        let identity = SovereignIdentity::new(
            "did:bostrom:bostrom18...".to_string(),
            "organic_cpu_001".to_string(),
        );
        
        assert!(identity.is_ok());
        assert_eq!(identity.unwrap().state, IdentityState::Unbound);
    }
    
    #[test]
    fn test_invalid_did_format() {
        let identity = SovereignIdentity::new(
            "did:invalid:notbostrom".to_string(),
            "organic_cpu_001".to_string(),
        );
        
        assert!(identity.is_err());
    }
    
    #[test]
    fn test_identity_binding() {
        let mut identity = SovereignIdentity::new(
            "did:bostrom:bostrom18...".to_string(),
            "organic_cpu_001".to_string(),
        ).unwrap();
        
        assert!(identity.bind_to_did("public_key_placeholder".to_string()).is_ok());
        assert_eq!(identity.state, IdentityState::Bound);
    }
}
