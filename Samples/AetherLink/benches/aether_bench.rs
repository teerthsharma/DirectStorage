use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aether_kernel::AetherLinkKernel;

fn bench_aether_link(c: &mut Criterion) {
    let mut kernel = AetherLinkKernel::new(
        0.5,
        0.1,
        [0.1, 0.2, 0.3],
        0.05
    );
    let lba_stream = vec![100, 101, 102, 105, 110, 200, 205]; // Dummy stream

    c.bench_function("process_io_cycle_optimized", |b| b.iter(|| {
        kernel.process_io_cycle(black_box(&lba_stream))
    }));

    c.bench_function("extract_telemetry_optimized", |b| b.iter(|| {
        kernel.extract_telemetry(black_box(&lba_stream))
    }));
    
    let features = kernel.extract_telemetry(&lba_stream);
    c.bench_function("prepare_quantum_state_optimized", |b| b.iter(|| {
        kernel.prepare_quantum_state(black_box(features))
    }));
}

fn bench_io_hook(c: &mut Criterion) {
    // We can't easily benchmark actual IO in this environment without a valid file/handle.
    // However, we can benchmark the overhead of the 'open' call failure or structure init.
    // For now, let's just ensure it compiles by referencing it.
    c.bench_function("io_hook_init_overhead", |b| b.iter(|| {
         let _ = aether_kernel::io_hook::WindowsIoBridge::open_nvme_namespace("INVALID_PATH");
    }));
}

criterion_group!(benches, bench_aether_link, bench_io_hook);
criterion_main!(benches);

