//! Kernel Audit Log
//!
//! Immutable audit trail for all guard decisions, syscall rejections,
//! and neurorights violations. Anchored to Organichain/Bostrom.

use alloc::string::String;
use alloc::vec::Vec;
use crate::error::{IntegrationError, IntegrationResult};
use crate::evidence_anchors;

/// Audit entry types
#[derive(Clone, Debug, PartialEq)]
pub enum AuditEntryType {
    SyscallWrapped,
    SyscallRejected,
    GuardDecision,
    NeurorightsViolation,
    ModuleLoadApproved,
    ModuleLoadRejected,
    BootChainVerified,
    EvolveTokenConsumed,
}

/// Single audit entry
#[derive(Clone, Debug)]
pub struct AuditEntry {
    pub entry_type: AuditEntryType,
    pub timestamp: u64,
    pub did: String,
    pub cpu_instance_id: String,
    pub details: String,
    pub hash: String,
    pub anchored: bool,
    pub anchor_hash: Option<String>,
}

impl AuditEntry {
    pub fn new(
        entry_type: AuditEntryType,
        did: String,
        cpu_instance_id: String,
        details: String,
    ) -> Self {
        Self {
            entry_type,
            timestamp: 0,
            did,
            cpu_instance_id,
            details,
            hash: String::new(),
            anchored: false,
            anchor_hash: None,
        }
    }
    
    pub fn compute_hash(&mut self) -> IntegrationResult<()> {
        // In real implementation, compute SHA3 hash of entry contents
        if self.details.is_empty() {
            return Err(IntegrationError::InvalidAuditEntry);
        }
        self.hash = "audit_hash_placeholder".to_string();
        Ok(())
    }
    
    pub fn mark_anchored(&mut self, chain_hash: String) {
        self.anchored = true;
        self.anchor_hash = Some(chain_hash);
    }
}

/// Kernel audit log (append-only)
#[derive(Clone, Debug)]
pub struct KernelAuditLog {
    pub entries: Vec<AuditEntry>,
    pub last_anchor_hash: Option<String>,
    pub total_entries: u64,
    pub anchored_entries: u64,
}

impl KernelAuditLog {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            last_anchor_hash: None,
            total_entries: 0,
            anchored_entries: 0,
        }
    }
    
    pub fn append(&mut self, mut entry: AuditEntry) -> IntegrationResult<()> {
        entry.compute_hash()?;
        entry.timestamp = self.total_entries; // Would be actual timestamp
        
        self.entries.push(entry);
        self.total_entries += 1;
        
        Ok(())
    }
    
    pub fn log_syscall_wrapped(&mut self, did: String, cpu_id: String, syscall: &str) -> IntegrationResult<()> {
        let entry = AuditEntry::new(
            AuditEntryType::SyscallWrapped,
            did,
            cpu_id,
            format!("Syscall wrapped: {}", syscall),
        );
        self.append(entry)
    }
    
    pub fn log_syscall_rejected(&mut self, did: String, cpu_id: String, syscall: &str, reason: &str) -> IntegrationResult<()> {
        let entry = AuditEntry::new(
            AuditEntryType::SyscallRejected,
            did,
            cpu_id,
            format!("Syscall rejected: {} - {}", syscall, reason),
        );
        self.append(entry)
    }
    
    pub fn log_neurorights_violation(&mut self, did: String, cpu_id: String, violation: &str) -> IntegrationResult<()> {
        let entry = AuditEntry::new(
            AuditEntryType::NeurorightsViolation,
            did,
            cpu_id,
            format!("Neurorights violation: {}", violation),
        );
        self.append(entry)
    }
    
    pub fn anchor_to_chain(&mut self, chain_hash: String) -> IntegrationResult<()> {
        self.last_anchor_hash = Some(chain_hash.clone());
        
        for entry in &mut self.entries {
            if !entry.anchored {
                entry.mark_anchored(chain_hash.clone());
                self.anchored_entries += 1;
            }
        }
        
        Ok(())
    }
    
    pub fn get_entries_by_type(&self, entry_type: &AuditEntryType) -> Vec<&AuditEntry> {
        self.entries
            .iter()
            .filter(|e| &e.entry_type == entry_type)
            .collect()
    }
    
    pub fn get_unanchored_entries(&self) -> Vec<&AuditEntry> {
        self.entries
            .iter()
            .filter(|e| !e.anchored)
            .collect()
    }
    
    pub fn export_audit_trail(&self) -> String {
        format!("AuditTrail: {} total, {} anchored, last_anchor: {:?}",
                self.total_entries, self.anchored_entries, self.last_anchor_hash)
    }
    
    pub fn to_hex_anchor(&self) -> String {
        format!("0xaudit{:08x}", self.total_entries as u32)
    }
}

impl Default for KernelAuditLog {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_audit_log_append() {
        let mut log = KernelAuditLog::new();
        
        let entry = AuditEntry::new(
            AuditEntryType::SyscallWrapped,
            "did:bostrom:test".to_string(),
            "cpu_001".to_string(),
            "Test entry".to_string(),
        );
        
        assert!(log.append(entry).is_ok());
        assert_eq!(log.total_entries, 1);
    }
    
    #[test]
    fn test_audit_log_anchor() {
        let mut log = KernelAuditLog::new();
        
        log.log_syscall_wrapped(
            "did:bostrom:test".to_string(),
            "cpu_001".to_string(),
            "open",
        ).unwrap();
        
        assert!(log.anchor_to_chain("chain_hash_placeholder".to_string()).is_ok());
        assert_eq!(log.anchored_entries, 1);
    }
    
    #[test]
    fn test_audit_log_filter_by_type() {
        let mut log = KernelAuditLog::new();
        
        log.log_syscall_wrapped(
            "did:bostrom:test".to_string(),
            "cpu_001".to_string(),
            "open",
        ).unwrap();
        
        log.log_neurorights_violation(
            "did:bostrom:test".to_string(),
            "cpu_001".to_string(),
            "Test violation",
        ).unwrap();
        
        let wrapped = log.get_entries_by_type(&AuditEntryType::SyscallWrapped);
        assert_eq!(wrapped.len(), 1);
        
        let violations = log.get_entries_by_type(&AuditEntryType::NeurorightsViolation);
        assert_eq!(violations.len(), 1);
    }
}
