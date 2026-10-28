---
name: verify
description: Drive the isolated terrain probe CLI and capture native/Rust observations.
---

Scope: reconstruction/terrain only; do not edit shared AGENTS, Cargo lock or building modules.

Build (not a verification verdict):
`cargo build --release --manifest-path games_test/reverse-engineering/tiny-glade/reconstruction/terrain/Cargo.toml --bin terrain-probe`

Drive the actual stdin/stdout executable with:
`tools/venv/bin/python games_test/reverse-engineering/tiny-glade/evidence/terrain-editing/milestones/M1/verify_rust_replay.py`

The driver executes original RNE Edit replay instructions and sends identical inputs to the compiled Rust CLI. It captures input/native output/Rust output in the M1 evidence directory, validates exact payload bytes and detects lack of native return. Read rust-replay-diff.json; never infer success from build or old reports. Its finite Edit-only scope is not complete terrain/GPU/undo acceptance.

CLI formats use hexadecimal f32 bits; replay accepts direction, index, four endpoint values, progress and event ID. Probe malformed rows at the executable boundary and capture stderr/exit when tools permit; if permission blocks the adjacent probe, say it was not exercised.

After implementation changes rebuild and rerun; source, binary and independent Cargo.lock hashes in evidence must match the current files. No unit test/typecheck runs substitute for this CLI observation.
