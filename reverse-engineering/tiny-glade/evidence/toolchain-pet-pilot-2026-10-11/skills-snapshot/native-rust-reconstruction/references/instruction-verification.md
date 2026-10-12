# Original-instruction differential verification

## Bind the oracle and candidate

Use the identified original artifact for execution. A patched analysis copy can supply static evidence but is not automatically the execution oracle. Record the executed binary digest, instruction entry/exit, candidate binary/build mode/target, relevant Rust source digests, manifest/lock digests and verification inputs.

Select comparison semantics from the behavior being claimed: exact bytes/bits for state projections and instance records, ordered events/cells for ordering policies, or explicitly justified tolerances for approximate outputs. Never weaken an exact comparison just to hide a failure.

## Make execution boundaries explicit

An emulator harness may supply resolved objects, preallocated output storage or library bridges. Document every bridge and skipped range. Execute the private business algorithm under test; do not inject its expected geometry, RNG state or output and claim an end-to-end proof.

Library allocator, hash lookup, ECS query or CRT boundaries can be bridged for a resolved business proof. That proof does not establish the excluded lookup, allocation failure, original scheduler or cross-platform libm equivalence. A separate real dependency integration test establishes the reconstructed interface behavior.

## Choose discriminating inputs

Cover business modes, edits, undo direction, missing objects, threshold boundaries and repeated/closing samples as relevant. Check enum-sized writes and preserved bytes; union payload/padding must not be zeroed when the original does not write it. Recover guards from actual loaded fields rather than guessed names.

For floating point preserve evidenced evaluation order, ordered/unordered branches and dependency call boundaries. Same dependency version does not imply x86/ARM or CRT bit equality. Retain first mismatching case/record/field/bits, distinguish business arithmetic from library/compiler differences, and investigate that boundary before changing code. Do not rewrite a dependency to mask an unexplained difference.

## Provenance and acceptance

Keep a narrow probe's dependency set accurate. Whole-workspace acceptance can separately bind all workspace sources. If an input hash changes, rerun the affected proof or retain its original source snapshot and mark it historical; never update report hashes without execution. Even formatting-only edits can invalidate byte-based source provenance, so avoid formatting unrelated verified files.

Save reproducible commands, seeds/input fixtures, success/failure counts and limits. Distinguish schema/static comparison, native execution, resolved composition and continuous runtime integration. Counts may overlap and do not form a product coverage denominator. Preserve failures alongside passed functional tests.
