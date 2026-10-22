//! Explicit macOS/platform host adapter. This entry does not pretend Steam
//! initialized successfully or replace the original game's assets.
#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL_ALLOCATOR: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() -> anyhow::Result<()> {
    // SAFETY: this is the first application operation at process entry,
    // before any application worker or Vulkan instance can be created.
    unsafe {
        std::env::set_var("MVK_CONFIG_USE_METAL_ARGUMENT_BUFFERS", "1");
    }
    tiny_glade::main::standalone_main()
}
