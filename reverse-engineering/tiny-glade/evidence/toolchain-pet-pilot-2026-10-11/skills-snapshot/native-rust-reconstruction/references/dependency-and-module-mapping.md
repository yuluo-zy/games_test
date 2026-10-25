# Dependency filtering and original-module mapping

## Classify before implementing

Compare three builds when supplied; use the actual number available without inventing missing metadata. For each baseline retain binary/debug digest, source paths/checksums, package version/source/commit and available compiler/engine information. Missing lockfiles remain unknown; debug registry/git paths may establish provenance without establishing every version or feature.

Classify a candidate by its definition and callers:

- Confirmed standard library or external dependency: reuse matching public APIs. Recover the application's argument selection, composition, event ordering and failure policy, not the dependency's internal implementation.
- Private/local package: inspect ownership and role. Local SDKs, allocators and engine code are not automatically business code; application-owned geometry utilities are not automatically excluded because they are Cargo packages.
- Derived/generic code: a container/query/serializer symbol containing an application type does not make its algorithm application-owned. The application's serialization schema or component tuple may still be recoverable from that boundary.
- Unresolved attribution: retain a candidate and identify the next discriminating call site or descriptor. Do not silently force a dependency/business classification.

Matching source checksums do not prove a library boundary; changed instructions do not prove new business logic. Compiler changes, inlining and generic specialization can change addresses or bytes without semantic change. Dehashed names are candidates only. Shared-address aliases require actual call parameters, descriptors and control flow; a decompiler's preferred alias may be wrong.

## Preserve the mapping

In the project ledger record, as evidence permits:

`baseline/digest → original crate + source path/checksum → symbol/address or executed range → caller/consumer → reconstructed module/API → verification report → confidence/gaps`

Separate original source-path metadata, inferred source attribution, recovered behavior and author source text. Initially grouping reconstructed code by a coherent business chain is acceptable. A later split into original crates must preserve APIs and verified behavior; do not restructure solely for visual similarity.

Keep baseline-specific layouts and behavior adapters explicit. A constructor or leaf matching between builds does not establish equivalence of all callers, serialization or lifecycle. Verify each claimed reuse at its actual boundary.

## Select the next useful work

Prioritize unresolved producers/consumers that connect existing modules. An accurate leaf with no authoritative input, event propagation or cleanup is partial recovery. Keep dependency exclusions separate from business coverage and never derive whole-product completion percentages from package/function counts.
