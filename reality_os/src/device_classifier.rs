//! Device Classification for Domain Enforcement
//!
//! Classifies all devices as INNER or OUTER domain, enforcing strict
//! access control at the kernel level.

use alloc::string::String;
use alloc::vec::Vec;
use crate::error::{IntegrationError, IntegrationResult};
use crate::evidence_anchors;

/// Domain label for device classification
#[derive(Clone, Debug, PartialEq)]
pub enum DomainLabel {
    /// INNER: Direct neural coupling (BCI, EEG, nanoswarm)
    Inner,
    /// OUTER: Network, storage, display (corridor-safe only)
    Outer,
}

impl DomainLabel {
    pub fn to_string(&self) -> &'static str {
        match self {
            DomainLabel::Inner => "inner",
            DomainLabel::Outer => "outer",
        }
    }
    
    pub fn allows_raw_neural_data(&self) -> bool {
        match self {
            DomainLabel::Inner => true,
            DomainLabel::Outer => false,
        }
    }
}

/// Device classification entry
#[derive(Clone, Debug)]
pub struct DeviceEntry {
    pub device_path: String,
    pub domain: DomainLabel,
    pub device_type: String,
    pub allowed_operations: Vec<String>,
}

impl DeviceEntry {
    pub fn new(device_path: String, domain: DomainLabel, device_type: String) -> Self {
        Self {
            device_path,
            domain,
            device_type,
            allowed_operations: Vec::new(),
        }
    }
    
    pub fn add_allowed_operation(&mut self, op: &str) {
        if !self.allowed_operations.contains(&op.to_string()) {
            self.allowed_operations.push(op.to_string());
        }
    }
    
    pub fn is_operation_allowed(&self, op: &str) -> bool {
        self.allowed_operations.contains(&op.to_string())
    }
}

/// Device classifier service
#[derive(Clone, Debug)]
pub struct DeviceClassifier {
    pub devices: Vec<DeviceEntry>,
    pub inner_domain_count: usize,
    pub outer_domain_count: usize,
}

impl DeviceClassifier {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
            inner_domain_count: 0,
            outer_domain_count: 0,
        }
    }
    
    pub fn register_device(&mut self, entry: DeviceEntry) -> IntegrationResult<()> {
        // Prevent duplicate registration
        if self.devices.iter().any(|d| d.device_path == entry.device_path) {
            return Err(IntegrationError::DeviceAlreadyRegistered {
                path: entry.device_path,
            });
        }
        
        match entry.domain {
            DomainLabel::Inner => self.inner_domain_count += 1,
            DomainLabel::Outer => self.outer_domain_count += 1,
        }
        
        self.devices.push(entry);
        Ok(())
    }
    
    pub fn classify_path(&self, path: &str) -> IntegrationResult<DomainLabel> {
        for device in &self.devices {
            if path.starts_with(&device.device_path) {
                return Ok(device.domain.clone());
            }
        }
        
        // Default to OUTER for unknown paths
        Ok(DomainLabel::Outer)
    }
    
    pub fn get_inner_devices(&self) -> Vec<&DeviceEntry> {
        self.devices
            .iter()
            .filter(|d| d.domain == DomainLabel::Inner)
            .collect()
    }
    
    pub fn is_inner_domain_path(&self, path: &str) -> bool {
        match self.classify_path(path) {
            Ok(DomainLabel::Inner) => true,
            _ => false,
        }
    }
    
    pub fn to_hex_anchor(&self) -> String {
        format!("0xdevclass{:08x}", self.devices.len() as u32)
    }
}

impl Default for DeviceClassifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_device_classifier_registration() {
        let mut classifier = DeviceClassifier::new();
        
        let inner_device = DeviceEntry::new(
            "/dev/bci_inner_001".to_string(),
            DomainLabel::Inner,
            "BCI".to_string(),
        );
        
        assert!(classifier.register_device(inner_device).is_ok());
        assert_eq!(classifier.inner_domain_count, 1);
    }
    
    #[test]
    fn test_device_classification() {
        let mut classifier = DeviceClassifier::new();
        
        classifier.register_device(DeviceEntry::new(
            "/dev/bci_inner_001".to_string(),
            DomainLabel::Inner,
            "BCI".to_string(),
        )).unwrap();
        
        let domain = classifier.classify_path("/dev/bci_inner_001");
        assert_eq!(domain.unwrap(), DomainLabel::Inner);
        
        let domain = classifier.classify_path("/dev/unknown");
        assert_eq!(domain.unwrap(), DomainLabel::Outer);
    }
    
    #[test]
    fn test_duplicate_registration_blocked() {
        let mut classifier = DeviceClassifier::new();
        
        classifier.register_device(DeviceEntry::new(
            "/dev/bci_inner_001".to_string(),
            DomainLabel::Inner,
            "BCI".to_string(),
        )).unwrap();
        
        let result = classifier.register_device(DeviceEntry::new(
            "/dev/bci_inner_001".to_string(),
            DomainLabel::Inner,
            "BCI".to_string(),
        ));
        
        assert!(result.is_err());
    }
}
