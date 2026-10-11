# P01 integration (initial, B only)

Dependencies verified directly in B build-info/Cargo.lock: country-core 0.2.0; glam 0.29.3 (game vectors), dolly 0.6.0 (depends on glam 0.27.0), bevy_app/bevy_ecs/bevy_input/bevy_time 0.16.0; serde 1.0.200. Use matching APIs; do not merge the two glam versions implicitly. Winit is PounceLight fork 0.29.15 commit d03de0a0a881be9422c360083151b0e7a8a2582e.

No shared workspace edit is requested yet. Public exports and terrain/input/history adapters are not confirmed until actual camera procedures/callers are inspected. Existing main_camera/camera_rig source paths establish provenance, not Rust source text or full ABI.

B bound Ghidra DB is patched EXE SHA256 cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323. Execution oracle must be original tiny-glade.rne SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03. C v1.16.0 remains independently unverified.
