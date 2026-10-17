//! Windowed end-to-end fixture player. See window_lab/README.md.
#[cfg(any(windows, target_os = "macos"))]
#[path = "window_lab/app.rs"]
mod app;
#[path = "window_lab/fixture.rs"]
mod fixture;
#[cfg(any(windows, target_os = "macos"))]
#[path = "window_lab/metrics.rs"]
mod metrics;
#[cfg(any(windows, target_os = "macos"))]
#[path = "window_lab/multi_queue_scene.rs"]
mod multi_queue_scene;
#[cfg(windows)]
#[path = "window_lab/win32.rs"]
mod platform;
#[cfg(target_os = "macos")]
#[path = "window_lab/macos.rs"]
mod platform;
#[cfg(any(windows, target_os = "macos"))]
#[path = "window_lab/scene.rs"]
mod scene;
#[cfg(any(windows, target_os = "macos"))]
#[path = "window_lab/transfer_scene.rs"]
mod transfer_scene;

#[cfg(any(windows, target_os = "macos"))]
#[path = "window_lab/result_scene.rs"]
mod result_scene;
#[cfg(any(windows, target_os = "macos"))]
#[path = "window_lab/transient_scene.rs"]
mod transient_scene;

fn main() {
    #[cfg(any(windows, target_os = "macos"))]
    if let Err(error) = app::run() {
        eprintln!("Window Lab failed: {error}");
        std::process::exit(1);
    }
    #[cfg(not(any(windows, target_os = "macos")))]
    {
        eprintln!("Window Lab requires Windows or macOS and a Vulkan presentation device.");
        std::process::exit(2);
    }
}

#[cfg(any(windows, target_os = "macos"))]
#[path = "window_lab/showcase.rs"]
mod showcase;
