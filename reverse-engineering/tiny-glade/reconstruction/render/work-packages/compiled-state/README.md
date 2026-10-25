# Compiled-state CPU candidate

This isolated module recovers BatchCompiler flush/finish state, initial-resource index collection, and pre-pass transition decisions with a caller-supplied concurrency classifier. It uses standard Rust containers; no ash/vk-sync/Bevy implementation is recreated.

This is not an ABI mirror or complete Vulkan graph executor. The snapshot is an owned semantic projection, and does not represent the 80-byte synchronization scope. BatchCompiler new/flush passed 1600 original-entry CPU differential cases; complete graph/barrier runtime equivalence is unaccepted.

Evidence: evidence/render-recovery/parallel/compiled-state/findings.md, type-ledger.json, snapshot-schema.json, implementation-contract.json, generic-vs-closure-boundary.json, unknowns.json, verification.json.

Validation: rustc --edition 2021 --crate-type lib lib.rs and rustfmt --edition 2021 --check lib.rs runner.rs passed. Existing workspace/python/Scripts/python.exe runs verify-batch.py: 1600 cases, zero failures. Vec growth, padding, GPU and other candidate behaviors are outside that oracle. No dependencies were installed.
