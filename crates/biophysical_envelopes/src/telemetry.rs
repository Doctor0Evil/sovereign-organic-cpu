//! Biophysical Telemetry Structures
//!
//! Standardized data structures for EEG, HRV, thermal, and other
//! biophysical telemetry inputs.

use alloc::string::String;
use alloc::vec::Vec;

/// EEG snapshot with frequency band powers
#[derive(Clone, Debug)]
pub struct EEGSnapshot {
    pub alpha: f32,
    pub beta: f32,
    pub gamma: f32,
    pub theta: f32,
    pub delta: f32,
    pub timestamp: u64,
}

impl EEGSnapshot {
    pub fn new(alpha: f32, beta: f32, gamma: f32, theta: f32, delta: f32) -> Self {
        Self {
            alpha,
            beta,
            gamma,
            theta,
            delta,
            timestamp: 0,
        }
    }
    
    pub fn theta_beta_ratio(&self) -> f32 {
        if self.beta == 0.0 {
            return 0.0;
        }
        self.theta / self.beta
    }
    
    pub fn alpha_gamma_ratio(&self) -> f32 {
        if self.gamma == 0.0 {
            return 0.0;
        }
        self.alpha / self.gamma
    }
    
    pub fn cognitive_load_score(&self) -> f32 {
        let tb_ratio = self.theta_beta_ratio();
        let ag_ratio = self.alpha_gamma_ratio();
        
        (tb_ratio + (1.0 - ag_ratio)) / 2.0
    }
}

/// HRV snapshot with time and frequency domain metrics
#[derive(Clone, Debug)]
pub struct HRVSnapshot {
    pub rmssd: f32,
    pub sdnn: f32,
    pub lf: f32,
    pub hf: f32,
    pub lf_hf_ratio: f32,
}

impl HRVSnapshot {
    pub fn new(rmssd: f32, sdnn: f32, lf: f32, hf: f32) -> Self {
        let lf_hf_ratio = if hf == 0.0 { 0.0 } else { lf / hf };
        
        Self {
            rmssd,
            sdnn,
            lf,
            hf,
            lf_hf_ratio,
        }
    }
    
    pub fn hrv_score(&self) -> f32 {
        let rmssd_normalized = (self.rmssd / 100.0).min(1.0);
        let sdnn_normalized = (self.sdnn / 150.0).min(1.0);
        let lf_hf_optimal = if self.lf_hf_ratio > 1.0 && self.lf_hf_ratio < 3.0 { 1.0 } else { 0.5 };
        
        (rmssd_normalized + sdnn_normalized + lf_hf_optimal) / 3.0
    }
}

/// Thermal snapshot
#[derive(Clone, Debug)]
pub struct ThermalSnapshot {
    pub core_temp: f32,
    pub skin_temp: f32,
    pub delta_from_baseline: f32,
}

impl ThermalSnapshot {
    pub fn new(core: f32, skin: f32, baseline: f32) -> Self {
        Self {
            core_temp: core,
            skin_temp: skin,
            delta_from_baseline: core - baseline,
        }
    }
    
    pub fn is_safe(&self) -> bool {
        self.delta_from_baseline <= 0.5
    }
}

/// Complete biophysical telemetry bundle
#[derive(Clone, Debug)]
pub struct BiophysicalTelemetry {
    pub eeg: EEGSnapshot,
    pub hrv: HRVSnapshot,
    pub thermal: ThermalSnapshot,
    pub timestamp: u64,
    pub session_id: String,
}

impl BiophysicalTelemetry {
    pub fn new(eeg: EEGSnapshot, hrv: HRVSnapshot, thermal: ThermalSnapshot) -> Self {
        Self {
            eeg,
            hrv,
            thermal,
            timestamp: 0,
            session_id: String::new(),
        }
    }
    
    pub fn validate(&self) -> bool {
        self.thermal.is_safe() && self.eeg.cognitive_load_score() < 0.8
    }
}
