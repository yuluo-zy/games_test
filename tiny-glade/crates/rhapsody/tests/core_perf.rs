//! Manual allocation/poll/host-write throughput probe.
//! `cargo test -p rhapsody --release --test core_perf -- --ignored --nocapture`
use ash::vk;
use rhapsody::device::{Device, DeviceConfig};
use rhapsody::raw::buffer::{Buffer, BufferDesc};
use std::time::Instant;

fn device() -> Option<std::sync::Arc<Device>> {
    Device::new_headless(&DeviceConfig::headless()).ok()
}

#[test]
#[ignore = "manual performance probe"]
fn core_perf_probe() {
    let Some(device) = device() else {
        eprintln!("SKIP: no Vulkan device");
        return;
    };
    let desc = BufferDesc::new_gpu_only(4096, vk::BufferUsageFlags::STORAGE_BUFFER);
    // Warm one native block.
    drop(Buffer::new(device.clone(), &desc).unwrap());
    const LIVE: usize = 4000;
    let started = Instant::now();
    let live: Vec<_> = (0..LIVE)
        .map(|_| Buffer::new(device.clone(), &desc).unwrap())
        .collect();
    let alloc = started.elapsed();
    const POLLS: usize = 200_000;
    let started = Instant::now();
    for _ in 0..POLLS {
        std::hint::black_box(device.poll().unwrap());
    }
    let poll = started.elapsed();
    let started = Instant::now();
    drop(live);
    device.try_collect_all_completed().unwrap();
    let free = started.elapsed();
    let host = Buffer::new(
        device.clone(),
        &BufferDesc::new_cpu_to_gpu(1 << 16, vk::BufferUsageFlags::UNIFORM_BUFFER),
    )
    .unwrap();
    const WRITES: usize = 100_000;
    let bytes = [7u8; 64];
    let started = Instant::now();
    for index in 0..WRITES {
        host.write_bytes(((index % 1000) * 64) as u64, &bytes)
            .unwrap();
    }
    let write = started.elapsed();
    println!(
        "PERF alloc_ns_per_buffer={} poll_ns={} free_collect_ns_per_buffer={} write64_ns={}",
        alloc.as_nanos() / LIVE as u128,
        poll.as_nanos() / POLLS as u128,
        free.as_nanos() / LIVE as u128,
        write.as_nanos() / WRITES as u128,
    );
}
