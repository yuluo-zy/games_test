# R5C temporal CPU candidate

Recovered semantic state projection from original single buffer/texture, texture pair, parity and export entrypoints. Dependencies reuse original Ustr1.0.0 and original-lock package versions/checksums. No Vulkan or interning implementation is reconstructed.

Raw texture0/raw buffer1 export to2/access0x37. Exported records can import on next frame; raw same-frame duplicate requests return error. Texture extent changes recreate, fixed descriptor mismatch panics, buffer descriptor changes are not compared. Pair names are base:0/base:1 and new per-base parity determines order; failure does not roll back first creation.

Run cargo test --locked --offline, cargo fmt --check and cargo clippy --locked --offline --all-targets -- -D warnings. Seven CPU tests cover multi-frame cycles and failure retention. Exact ABI/reset/eviction/drop/compile unwind and buffer-pair API remain unknown; no GPU or whole-game acceptance is claimed.
