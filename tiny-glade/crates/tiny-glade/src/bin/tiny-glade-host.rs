//! Explicit macOS/platform host adapter. This entry does not pretend Steam
//! initialized successfully or replace the original game's assets.
#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL_ALLOCATOR: mimalloc::MiMalloc = mimalloc::MiMalloc;

/// `TG_PROFILE_OUT=<file>` samples every thread's CPU time (997 Hz) for
/// `TG_PROFILE_SECS` (default 30) starting `TG_PROFILE_DELAY` seconds (default
/// 100) after launch, then writes collapsed stacks (`thread;frame;...;leaf N`,
/// flamegraph input) so one run supports any offline analysis.
#[cfg(feature = "profile")]
fn start_profiler() {
    let Some(out) = std::env::var_os("TG_PROFILE_OUT") else {
        return;
    };
    let secs = |name: &str, default: u64| {
        std::env::var(name)
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(default)
    };
    let (delay, length) = (secs("TG_PROFILE_DELAY", 100), secs("TG_PROFILE_SECS", 30));
    std::thread::spawn(move || {
        use std::io::Write;
        std::thread::sleep(std::time::Duration::from_secs(delay));
        let Ok(guard) = pprof::ProfilerGuardBuilder::default()
            .frequency(997)
            .blocklist(&["libc", "libgcc", "pthread", "vdso"])
            .build()
        else {
            eprintln!("profiler failed to start");
            return;
        };
        std::thread::sleep(std::time::Duration::from_secs(length));
        let Ok(report) = guard.report().build() else {
            return;
        };
        let Ok(mut file) = std::fs::File::create(&out) else {
            return;
        };
        for (frames, count) in &report.data {
            let mut line = frames.thread_name.clone();
            for frame in frames.frames.iter().rev() {
                // Inlined frames come innermost first; keep the outermost first.
                for symbol in frame.iter().rev() {
                    line.push(';');
                    line.push_str(&symbol.name().replace(';', ":"));
                }
            }
            let _ = writeln!(file, "{line} {count}");
        }
        eprintln!("profile written to {}", std::path::Path::new(&out).display());
    });
}

fn main() -> anyhow::Result<()> {
    #[cfg(feature = "profile")]
    start_profiler();
    // SAFETY: this is the first application operation at process entry,
    // before any application worker or Vulkan instance can be created.
    unsafe {
        std::env::set_var("MVK_CONFIG_USE_METAL_ARGUMENT_BUFFERS", "1");
    }
    tiny_glade::main::standalone_main()
}
