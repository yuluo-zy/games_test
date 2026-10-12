# R6 record/submit event projection

This dependency-free Rust candidate exposes typed `Event` and `Outcome` projections for three scoped original functions. Numeric command handles and receipts are explicitly fixture tokens, not live Vulkan values.

- `record`: sequential, empty resource lists, callbacks taken before invocation, partial failure and append/end/final-export decisions.
- `submit`: taken pending command vector, last-command validation/end, structured queue delegation and receipt propagation.
- `queue_call`: only the original backend-call/error/receipt tail with a prebuilt frame.

The matching original-PE harness and reports live in `evidence/render-recovery/parallel-batch-04/record-submit`. This crate does not implement GPU submission, library internals, complete queue construction, or production render modules.

Validation: `cargo test --offline --locked`; `cargo clippy --offline --locked --all-targets -- -D warnings`. The harness uses the existing workspace Python Unicorn environment and read-only R5 PE loader.
