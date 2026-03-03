//! Constitutional Audit Logging
//!
//! Records all neurorights violations and policy changes in an immutable
//! log that can be anchored to Organichain/Bostrom for external verification.

use alloc::string::String;
use alloc::vec::Vec;
use crate::invariants::InvariantViolation;
use crate::error::{KernelError, KernelResult};

/// Record of a constitutional violation
#[derive(Clone, Debug)]
pub struct ViolationRecord {
    pub violation: InvariantViolation,
    pub did: String,
    pub cpu_instance_id: String,
    pub anchored: bool,
    pub anchor_hash: Option<String>,
}

impl ViolationRecord {
    pub fn new(violation: InvariantViolation, did: String, cpu_instance_id: String) -> Self {
        Self {
            violation,
            did,
            cpu_instance_id,
            anchored: false,
            anchor_hash: None,
        }
    }
    
    pub fn mark_anchored(&mut self, hash: String) {
        self.anchored = true;
        self.anchor_hash = Some(hash);
    }
}

/// Immutable constitutional log
#[derive(Clone, Debug)]
pub struct ConstitutionalLog {
    pub records: Vec<ViolationRecord>,
    pub last_anchor_hash: Option<String>,
}

impl ConstitutionalLog {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
            last_anchor_hash: None,
        }
    }
    
    pub fn log_violation(&mut self, record: ViolationRecord) -> KernelResult<()> {
        // In real implementation, append to append-only log
        self.records.push(record);
        Ok(())
    }
    
    pub fn get_violations_by_right(&self, right: &crate::invariants::NeurorightType) -> Vec<&ViolationRecord> {
        self.records
            .iter()
            .filter(|r| &r.violation.right_type == right)
            .collect()
    }
    
    pub fn get_critical_violations(&self) -> Vec<&ViolationRecord> {
        self.records
            .iter()
            .filter(|r| r.violation.severity >= 8)
            .collect()
    }
    
    pub fn anchor_to_chain(&mut self, hash: String) {
        self.last_anchor_hash = Some(hash);
        for record in &mut self.records {
            if !record.anchored {
                // In real implementation, mark as anchored
            }
        }
    }
    
    pub fn export_audit_trail(&self) -> String {
        // In real implementation, serialize to JSON for external audit
        format!("AuditTrail: {} records, last_anchor: {:?}", 
                self.records.len(), self.last_anchor_hash)
    }
}

impl Default for ConstitutionalLog {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::invariants::{InvariantViolation, NeurorightType};
    
    #[test]
    fn test_log_violation() {
        let mut log = ConstitutionalLog::new();
        let violation = InvariantViolation::new(
            NeurorightType::CognitiveLiberty,
            "Test violation".to_string(),
            "hash123".to_string(),
        );
        let record = ViolationRecord::new(violation, "did:test".to_string(), "cpu001".to_string());
        
        assert!(log.log_violation(record).is_ok());
        assert_eq!(log.records.len(), 1);
    }
    
    #[test]
    fn test_filter_critical() {
        let mut log = ConstitutionalLog::new();
        let mut violation = InvariantViolation::new(
            NeurorightType::MentalPrivacy,
            "Critical breach".to_string(),
            "hash456".to_string(),
        );
        violation.severity = 10;
        
        let record = ViolationRecord::new(violation, "did:test".to_string(), "cpu001".to_string());
        log.log_violation(record).unwrap();
        
        let critical = log.get_critical_violations();
        assert_eq!(critical.len(), 1);
    }
}
