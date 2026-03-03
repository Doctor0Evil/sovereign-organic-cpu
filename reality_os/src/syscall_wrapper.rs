//! Syscall Wrapper for Guard Enforcement
//!
//! Wraps all syscalls that touch INNER domain devices or sensitive resources
//! with OrganicCPU guard checks before allowing execution.

use alloc::string::String;
use crate::kernel_guard::KernelGuard;
use crate::device_classifier::DomainLabel;
use crate::error::{IntegrationError, IntegrationResult};
use crate::evidence_anchors;

/// Wrapped syscall types
#[derive(Clone, Debug, PartialEq)]
pub enum WrappedSyscall {
    Open,
    Read,
    Write,
    Mmap,
    Ioctl,
    Socket,
    Connect,
    Send,
    Receive,
}

impl WrappedSyscall {
    pub fn requires_guard_check(&self, domain: &DomainLabel) -> bool {
        match (self, domain) {
            // INNER domain always requires guard check
            (_, DomainLabel::Inner) => true,
            // OUTER domain syscalls may require checks depending on type
            (WrappedSyscall::Mmap, DomainLabel::Outer) => true,
            (WrappedSyscall::Ioctl, DomainLabel::Outer) => true,
            (WrappedSyscall::Socket, DomainLabel::Outer) => true,
            _ => false,
        }
    }
    
    pub fn to_string(&self) -> &'static str {
        match self {
            WrappedSyscall::Open => "open",
            WrappedSyscall::Read => "read",
            WrappedSyscall::Write => "write",
            WrappedSyscall::Mmap => "mmap",
            WrappedSyscall::Ioctl => "ioctl",
            WrappedSyscall::Socket => "socket",
            WrappedSyscall::Connect => "connect",
            WrappedSyscall::Send => "send",
            WrappedSyscall::Receive => "receive",
        }
    }
}

/// Syscall wrapper with guard enforcement
#[derive(Clone, Debug)]
pub struct SyscallWrapper {
    pub guard: KernelGuard,
    pub wrapped_count: u64,
    pub rejected_count: u64,
}

impl SyscallWrapper {
    pub fn new(guard: KernelGuard) -> Self {
        Self {
            guard,
            wrapped_count: 0,
            rejected_count: 0,
        }
    }
    
    pub fn wrap_open(&mut self, path: &str, domain: &DomainLabel) -> IntegrationResult<()> {
        self.wrapped_count += 1;
        
        if !self.guard.service.is_active() {
            return Err(IntegrationError::GuardServiceNotActive);
        }
        
        // INNER domain paths require additional vetting
        if *domain == DomainLabel::Inner {
            // In real implementation, check against device whitelist
            if !path.starts_with("/dev/bci_inner") {
                return Err(IntegrationError::InnerDomainAccessDenied {
                    path: path.to_string(),
                });
            }
        }
        
        Ok(())
    }
    
    pub fn wrap_mmap(&mut self, addr: u64, size: usize, domain: &DomainLabel) -> IntegrationResult<()> {
        self.wrapped_count += 1;
        
        if !self.guard.service.is_active() {
            return Err(IntegrationError::GuardServiceNotActive);
        }
        
        // Block mmap to INNER domain memory regions
        if *domain == DomainLabel::Inner {
            return Err(IntegrationError::InnerDomainMmapBlocked);
        }
        
        Ok(())
    }
    
    pub fn wrap_ioctl(&mut self, fd: i32, request: u64, domain: &DomainLabel) -> IntegrationResult<()> {
        self.wrapped_count += 1;
        
        if !self.guard.service.is_active() {
            return Err(IntegrationError::GuardServiceNotActive);
        }
        
        // INNER domain ioctl requires guard approval
        if *domain == DomainLabel::Inner {
            // In real implementation, verify request against allowed ioctl codes
        }
        
        Ok(())
    }
    
    pub fn wrap_socket(&mut self, domain_type: i32, socket_type: i32) -> IntegrationResult<()> {
        self.wrapped_count += 1;
        
        if !self.guard.service.is_active() {
            return Err(IntegrationError::GuardServiceNotActive);
        }
        
        // Block raw sockets that could bypass network stack
        if socket_type == 3 {
            // SOCK_RAW
            return Err(IntegrationError::RawSocketBlocked);
        }
        
        Ok(())
    }
    
    pub fn reject_syscall(&mut self, syscall: &WrappedSyscall, reason: &str) {
        self.rejected_count += 1;
        // In real implementation, log to audit trail
    }
    
    pub fn get_rejection_rate(&self) -> f32 {
        if self.wrapped_count == 0 {
            return 0.0;
        }
        self.rejected_count as f32 / self.wrapped_count as f32
    }
    
    pub fn to_hex_anchor(&self) -> String {
        format!("0xsyscall{:08x}", self.wrapped_count as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_syscall_wrapper_creation() {
        let guard = KernelGuard::load().unwrap();
        let wrapper = SyscallWrapper::new(guard);
        
        assert_eq!(wrapper.wrapped_count, 0);
        assert_eq!(wrapper.rejected_count, 0);
    }
    
    #[test]
    fn test_wrap_open_inner_domain() {
        let guard = KernelGuard::load().unwrap();
        let mut wrapper = SyscallWrapper::new(guard);
        
        let result = wrapper.wrap_open("/dev/bci_inner_001", &DomainLabel::Inner);
        assert!(result.is_ok());
        
        let result = wrapper.wrap_open("/dev/unknown", &DomainLabel::Inner);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_wrap_socket_raw_blocked() {
        let guard = KernelGuard::load().unwrap();
        let mut wrapper = SyscallWrapper::new(guard);
        
        let result = wrapper.wrap_socket(2, 3); // AF_INET, SOCK_RAW
        assert!(result.is_err());
    }
}
