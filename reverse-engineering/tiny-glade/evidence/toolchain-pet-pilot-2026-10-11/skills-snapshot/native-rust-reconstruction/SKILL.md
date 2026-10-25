---
name: native-rust-reconstruction
description: Recover native application business logic into an evidence-backed Rust project using release binaries, debug metadata and cross-version comparisons. Use for dependency filtering, original-module mapping, machine-code differential verification and Bevy business-chain integration; not ordinary source-only refactoring.
---

# Native Rust Reconstruction

Deliver executable business behavior whose provenance and verification boundary can be inspected. Preserve the user's target version and authorized scope. Reuse available disassembler/provider skills for binary operations; this skill adds reconstruction decisions and acceptance discipline, not a mandatory tool installation.

## Start from retained evidence

Read applicable repository guidance, the current findings ledger and next unresolved consumer. Confirm binary/debug identities and the existing Rust workspace before choosing work. Reuse matching analysis databases. A debug source path/checksum is provenance, not recovered author source text.

Choose a bounded business chain: authoritative input → state/relationship updates → consumer output → edit/replay → removal. Record which links already have evidence and implement the highest-value unresolved link. Do not replace a requested full-chain task with a collection of isolated leaf probes.

## Read only the needed reference

- Before selecting functions, mapping modules or comparing releases, read [dependency-and-module-mapping.md](references/dependency-and-module-mapping.md).
- Before executing a native oracle, changing numerical boundaries or accepting a recovered implementation, read [instruction-verification.md](references/instruction-verification.md).
- When recovering Bevy registration, scheduling or lifecycle consumers, read [bevy-business-chain.md](references/bevy-business-chain.md).
- Before calling analysis tools or coordinating concurrent workers, read [tools-and-parallel-workers.md](references/tools-and-parallel-workers.md); follow the project's AI tool guide for executable commands and verified parameters.

## Reuse bounded workflow skills

- For repeated original source/inline and raw instruction lookups, use `native-evidence-pack`; bind its cache to both binary and debug identities.
- For a known provider's repeated MCP queries, use `native-tool-batch`; preserve database identity limits, revisions and nested errors.
- For parallel worker integration and remaining consumers, use `native-module-handoff`; its audit checks references, not semantic truth.

Use these when their repeated operations occur. Do not make a new tool pipeline a prerequisite for a business gap already answerable from retained evidence.

## Token control is part of the workflow

- Begin with symbol/module metadata, call sites and a concrete hypothesis. Fetch only the function or instruction range needed to resolve it. Prefer bounded summaries over full decompiler or assembly dumps.
- Search indexes with targeted terms; summarize large JSON programmatically. Keep full tool outputs on disk with stable evidence references and show compact results.
- Batch independent lookups; keep dependent analysis and mutations sequential. Avoid reopening a database, repeating whole-program analysis or rereading an unchanged skill/reference.
- Reuse probes and verifiers. Rerun a report when its measured inputs, source dependency set, tool boundary or unresolved failure changes. Bind reports to the actual relevant inputs; do not inflate a narrow proof into workspace acceptance.
- Preserve a short handoff ledger: proven behavior, corrected interpretations, unresolved boundary, next consumer, source/report locations. Do not count tokens saved, scripts written or functions decompiled as recovery progress.

When an isolated harness introduces a new lockfile, run `scripts/compare_cargo_locks.py ORIGINAL.lock CANDIDATE.lock --output comparison.json` before claiming original dependency provenance. Direct pins alone do not freeze transitive packages. Use a copy of the original lock as a resolution seed, preserving release inputs, then compare again after Cargo prunes/resolves it. Platform-only or incompatible packages stay explicit mismatches; matching locks do not prove fork behavior equivalence.

## Keep general method separate from project facts

Keep addresses, layouts, offsets, version-specific enum values, binary paths and numerical failures in the target project's evidence. Skills retain the method and pitfalls. Map original modules to reconstructed Rust modules explicitly; do not claim an original directory structure, exact ABI or fork schedule merely because the reconstructed project builds.

Before finishing, verify the authorized chain against original behavior and actual integration where feasible. Report separately: dependency reuse, inferred schema, recovered business logic, native differential scope, runtime integration and remaining system gaps. Preserve failed comparisons; compilation and local probes cannot substitute for full target acceptance.
