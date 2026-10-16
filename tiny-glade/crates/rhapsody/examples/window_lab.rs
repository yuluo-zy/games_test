//! Windowed end-to-end fixture player. See window_lab/README.md.
#[cfg(windows)]
#[path = "window_lab/app.rs"]
mod app;
#[path = "window_lab/fixture.rs"]
mod fixture;
#[cfg(windows)]
#[path = "window_lab/win32.rs"]
mod win32;

fn main() {
    #[cfg(windows)]
    if let Err(error) = app::run() {
        eprintln!("Window Lab failed: {error}");
        std::process::exit(1);
    }
    #[cfg(not(windows))]
    {
        eprintln!("Window Lab V0 requires Windows and a Vulkan presentation device.");
        std::process::exit(2);
    }
}
