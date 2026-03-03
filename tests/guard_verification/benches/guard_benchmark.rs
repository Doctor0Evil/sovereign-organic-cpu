//! Guard Performance Benchmarks
//!
//! Measures latency and throughput of guard operations.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use organic_cpu_guard::{GuardContext, NeurorightsGuard};
use biophysical_envelopes::{RoHCalculator, BiophysicalEnvelope};

fn bench_roh_calculation(c: &mut Criterion) {
    c.bench_function("roh_calculation_standard", |b| {
        b.iter(|| {
            let mut calc = RoHCalculator::new();
            calc.add_eeg_load(black_box(0.3))
                .add_hrv_score(black_box(0.8))
                .add_thermal_delta(black_box(0.2))
                .add_duty_cycle(black_box(0.25))
                .add_cytokine_load(black_box(3.0), black_box(1.5));
            
            let _ = calc.build();
        })
    });
}

fn bench_guard_context_creation(c: &mut Criterion) {
    c.bench_function("guard_context_creation", |b| {
        b.iter(|| {
            let _ctx = GuardContext::new(
                black_box("did:bostrom:bostrom18...".to_string()),
                black_box("organic_cpu_001".to_string()),
            );
        })
    });
}

fn bench_envelope_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("envelope_validation");
    
    for roh_value in [0.1, 0.2, 0.25, 0.29].iter() {
        group.bench_with_input(
            BenchmarkId::new("roh_threshold", roh_value),
            roh_value,
            |b, &roh| {
                b.iter(|| {
                    let envelope = BiophysicalEnvelope::new(
                        black_box(roh),
                        black_box(0.3),
                        black_box(biophysical_envelopes::LifeforceBand::Baseline),
                        black_box(biophysical_envelopes::EcoImpactScore::new(
                            biophysical_envelopes::CEIM::new(0.1, 0.1, 0.1),
                            biophysical_envelopes::NanoKarma::new(50, 0.01, 0.9),
                            0.2,
                        )),
                    );
                    let _ = envelope.unwrap().is_safe();
                })
            },
        );
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_roh_calculation,
    bench_guard_context_creation,
    bench_envelope_validation,
);

criterion_main!(benches);
