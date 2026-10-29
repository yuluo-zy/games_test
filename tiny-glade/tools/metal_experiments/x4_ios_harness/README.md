# X4: minimal iOS harness (plan; not implemented)

Goal (METAL_DESIGN.md X4): run the static scene on an iPhone through the current Vulkan backend on
MoltenVK, and record frame time, thermal state and memory, to find out whether mobile is compute-,
bandwidth- or memory-bound and what budget D15 can set.

**This directory contains only the telemetry helper (`DeviceTelemetry.swift`, never compiled).** The
harness itself is not written: it cannot be written correctly yet because of the blockers below. Each
is a fact about the repository, not a guess.

## Blockers, in the order they bite

| # | Blocker | Evidence | What resolves it |
| --- | --- | --- | --- |
| 1 | **Assets and FMOD licence.** The renderer reads the original game's install directory at run time. iOS has no such directory; assets must ship in the app bundle. | `TG_INSTALL_ROOT` in `crates/tiny-glade/src/gui/gui_controls.rs:693`, tests under `tests/`; METAL_DESIGN.md D20 | A written decision on redistribution of the original assets and FMOD. A sideloaded personal-device run with your own copy is the narrowest case; it is still your call, not a technical one. |
| 2 | **The fixture is a test binary.** The headless scene lives in `tests/cpu_measurement/` of `original_gameplay_chain`, not in a library, so an iOS app cannot call it. | `crates/tiny-glade/tests/original_gameplay_chain.rs:6` (`mod cpu_measurement`), `Cargo.toml` `[[test]]` entries | Move the fixture body behind a library entry point (for example `tiny_glade::headless::run(config) -> Report`) used by both the test and the harness. A refactor of test code: do it as its own reviewed change. |
| 3 | **Windowing on iOS.** The workspace uses a patched winit fork; whether it supports iOS is unknown. The headless fixture needs no window, but a Vulkan instance on MoltenVK still needs a surface-less path (offscreen only is fine). | `Cargo.toml` `winit = { path = "third_party/winit-d03de0a0" }` | Offscreen-only harness avoids winit entirely; confirm `crates/tiny-glade` does not pull windowing into the library path used by the fixture. |
| 4 | **Audio.** `native-audio` (FMOD) is Windows-default and optional elsewhere. | `crates/tiny-glade/Cargo.toml:130` | Build the harness without `native-audio`. |
| 5 | **Linking MoltenVK on iOS.** There is no Vulkan loader on iOS; MoltenVK must be linked statically and the instance created without the loader. | `native/instance/mod.rs:56,74` load the Vulkan library dynamically (`Entry::load_from`, `Entry::load`) | `ash` has a `linked` feature that links the loader statically (from memory of the crate; not checked in this repo). Either way it is a code change in the instance creation path (`native/instance/mod.rs:56,74` use `Entry::load_from` / `Entry::load`). |
| 6 | **Rust toolchain.** The `aarch64-apple-ios` standard library must be installable (blocked on the machine this was written on; see X6). | `tools/metal_experiments/x6_apple_check.sh` | Run on a machine with access to `static.rust-lang.org`. |
| 7 | **Signing / Xcode project.** An app target with a code-signing identity and a device. | — | Yours. |

## Shape of the harness once blockers 1-5 are resolved

1. A Rust `staticlib` crate with one C ABI function that runs the (refactored) fixture for a fixed
   scene/preset and returns a JSON report string; no windowing.
2. A minimal Xcode project (single `UIViewController`-less app, `main.swift`) that calls it on a
   background thread, samples `Telemetry` (this directory) every second, and writes
   `report.json` + `telemetry.csv` to the app's Documents directory for retrieval with Finder / Xcode.
3. The same report schema as the Windows runs (`session.json` from the fixture) so
   `docs/performance/evidence-gpu-pass-20261027/analyze_gpu_passes.py` works unchanged.

## What to record (so runs are comparable)

- device model and iOS version; thermal state at start; battery level; low power mode off;
- scene, quality preset, warmup/samples, resolution;
- frame time distribution (mean, p50, p95, p99) and per-pass GPU time from the isolated run;
- `physFootprint` and `os_proc_available_memory` at start, at the end of warmup, and every second;
- for the sustained test: 30 minutes at the chosen preset; the thermal curve and the frame-time
  drift (first vs last 5 minutes) are the result, not a single average.

## Cheaper first step

X2 (`../x2_mac_moltenvk.py`) gives the Apple-GPU pass breakdown on a Mac without any of these blockers.
Do it first; X4 only adds what a phone's memory limit, thermals and bandwidth change.
