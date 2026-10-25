fn main() -> anyhow::Result<()> {
    // SAFETY: process-entry setup precedes CLI, Steam, logging, task-pool,
    // event-loop and renderer initialization. No application thread exists.
    unsafe {
        std::env::set_var("MVK_CONFIG_USE_METAL_ARGUMENT_BUFFERS", "1");
    }
    tiny_glade::main::main()
}
