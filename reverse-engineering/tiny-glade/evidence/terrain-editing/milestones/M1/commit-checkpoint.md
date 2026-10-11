# Terrain commit checkpoint — 2026-10-11

User requested committing/pushing the terrain work in progress. Shared AGENTS, workspace lockfiles and parallel building/camera/critters edits are excluded.

## Included changes

- `terrain-probe` points command.
- Original-RNE versus Rust resolved edit/point helper differential driver, inputs, native/Rust outputs and report: 1200 cases, zero differences in recorded run.
- `src/readback.rs`: proposed CPU readback/diff implementation. Not registered in lib.rs, not compiled/exercised by the package yet. No GPU computation or complete terrain acceptance claim.

## Pre-commit test observation

`cargo test --manifest-path .../reconstruction/terrain/Cargo.toml -p tg-terrain`: 6 passed, 1 failed.

Failure: `schema::tests::preserves_external_variant_and_field_names` compares serde_json Value representations of typed f32 numbers with the input decimal/f64 values. Observed `0.4` versus `0.4000000059604645`, and `0.8` versus `0.800000011920929`. This needs a later review of the test's numeric contract; neither schema compatibility nor original save-format roundtrip is declared passed.

Because the test command failed, the chained differential command did not execute in that invocation. Differential verification is run separately and its report remains the source of actual native/Rust results.

## Evidence whitespace

`git diff --check` reports trailing spaces on captured native/Rust CLI lines with no audio records. These are identical raw outputs produced by the existing output format; they are preserved as evidence rather than silently normalized. Source patch whitespace check passed.

## Pending

User will review later. M1 remains incomplete: schema test failure, readback module wiring/native verification, ID/exception edges, full state/runtime/GPU/history/save and A/C remain unclosed. No changes to other sessions' pending work.
