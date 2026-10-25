# M1 implementation status — 2026-10-11

## Delivered source

Independent `reconstruction/terrain` Cargo workspace, its own lock/target, tg-terrain package. No shared Cargo/AGENTS/building/DB changes.

Modules: schema (New/Edit/Move), stroke/ID, resolved edit rules, Edit replay observation/command, compiled stdin/stdout terrain-probe. This is an initial M1 implementation, not completed terrain or lifecycle/runtime/GPU.

## Executed evidence

Initial cargo test/build succeeded (before subsequent replay observation refactor). Current release probe rebuilt successfully after that refactor.

`verify_rust_replay.py` drove the compiled CLI with 512 original-RNE Edit replay inputs. Exact valid40-byte payload comparison: 512 cases, zero mismatches. Captures: replay-inputs.txt, replay-native-output.txt, replay-rust-output.txt and rust-replay-diff.json (source/binary/independent lock hashes). This is now original↔Rust, not the earlier Python formula model.

Scope excludes New/Move, NaN/Inf, allocation/queue growth, actual ECS resource resolution, UI, history transaction, GPU and A/C. It does not prove full state/edit/point paths, nor full undo/redo.

## Review/verification not yet passed

Long-running reverse and verify agents were launched with distinct file ownership and communication IDs, but repeated inference gateway 400/504 errors prevented useful results. The task metadata returned swapped display labels; prompts explicitly restored roles. No independent review verdict was obtained.

The requested code-review skill was attempted and denied by the permission classifier. Adjacent direct CLI input/error probing was also denied. These checks were not executed and must not be recorded PASS. Tool failures are not evidence of program correctness.

## Pending M1 completion

- Independent code review and fix/review cycle.
- Original-Rust diff for Delta/Absolute/Keep, point maintenance, ID/collision and non-finite/signed-zero boundaries.
- Check native-return termination on all harness paths and dependency identity.
- Resolve collision behavior and point/schema ownership contract with reverse agent.
- Actual edit CLI/error-path capture once permitted.

M1 remains `in_progress`; no milestone acceptance or complete terrain claim. M2..M8 have not been implemented.
