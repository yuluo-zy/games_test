---
name: native-module-handoff
description: Create and audit an evidence-linked recovery handoff for a bounded native business chain across versions. Use when parallel reverse engineering workers need to distinguish dependency reuse, recovered behavior, integration and unresolved consumers before merging Rust changes.
---

# Native Module Handoff

Pair with native-rust-reconstruction for reconstruction decisions. This skill produces the small contract ledger used between parallel workers, not automatic acceptance.

Start from an authoritative input and include state updates, output consumers, registration/edit replay and removal where applicable. Map original source owners to reconstructed files. Record A/B/C comparisons, dependency boundaries and unknown fields in project evidence, never in this generic skill. Library wrappers with business callbacks need separate dependency and game contracts.

Write a JSON ledger: `target_version`, `module`, `versions` (each version has `artifact_sha256`), `facts` (id, version, kind, evidence relative file, optional pointer), `links` (name, status, fact_ids, optional gaps), `dependencies`, and `rust_files`. Link status is one of `unknown`, `evidence_ready`, `implemented_unverified`, `verified`. Kind is `source_owner`, `instruction`, `behavior`, `dependency`, `runtime`, or `verification`. Verified links must cite both runtime and verification facts from the target version. Other-version observations support hypotheses, not target acceptance.

Run `scripts/audit_handoff.py LEDGER.json PROJECT_ROOT OUTPUT.json`. It checks references/files/version hashes, emits unresolved links and source fingerprints, and flags unsupported verified labels. It does not parse native instructions or attest that evidence content is true. Do not auto-fill an unknown field or schedule merely to pass an audit. Review conflicting worker facts before shared integration; retain both hypotheses with their evidence until resolved.

Hand off only the short audit plus exact evidence/source paths. Report newly recovered behavior, reused dependencies, current integration boundary and highest-value missing consumer. Count complete chain links, not symbols fetched, as progress.
