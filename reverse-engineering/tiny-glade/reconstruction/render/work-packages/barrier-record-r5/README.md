# R5 scoped barrier/record candidate

This independent, dependency-free Rust package restores only CPU transition decisions. It does not replace Vulkan, std, ash, vk-sync, or any original dependency. `pre` preserves no-predecessor/reused rules and concurrency; `initial` preserves drain-before-panic behavior; `export` preserves the final-batch gate and last-access fallback.

The corresponding original-PE instruction harnesses and validation reports are in `evidence/render-recovery/parallel-batch-03/barrier-record`. The export native oracle begins at an injected post-record frame, unlike the original-entry pre/initial/population scopes.

Run `cargo test --offline --locked --manifest-path Cargo.toml`. Oracle scripts require the existing workspace Python environment with Unicorn and the identity-matching original PE.
