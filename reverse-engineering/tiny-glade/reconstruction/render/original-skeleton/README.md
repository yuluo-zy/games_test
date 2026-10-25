# Original-renderer metadata hierarchy

These 93 PDB-observed `.rs` files reflect compiled contributing source paths from the original Tiny Glade build. They retain source SHA256 checksums and native function contribution metadata. They do not contain recovered author source or Rust signatures.

`crates/rhapsody` is now an isolated Cargo-checkable project topology. Its 55 observed source files remain evidence-only; three synthetic `mod.rs` files connect the PDB-observed hierarchy for Cargo. The synthetic files are listed separately in `crates/rhapsody/structure.json` and are not claimed as original source paths or recovered implementation.

Evidence and limitations: `../../../evidence/render-skeleton/findings.md`, `files.json`, `native-procedures.json`, and `report.json`. Source/function association uses line-table overlap and can include inline contributions; it does not establish primary ownership.

Generated with `scripts/render-skeleton/inventory.py`. Never replace these comments with invented signatures to claim complete source recovery.
