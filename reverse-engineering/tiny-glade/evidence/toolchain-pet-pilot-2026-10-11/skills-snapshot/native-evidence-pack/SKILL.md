---
name: native-evidence-pack
description: Collect bounded assembly and source/inline evidence from identity-bound native PE x86/x64 binaries and matching PDBs using LLVM, with batch symbolization, integrity-checked caches and measured cold/warm costs. Use for native reconstruction function packs; not source-only coding or behavioral validation.
---

# Native evidence pack

Produce a small, reusable evidence pack before requesting a heavyweight analysis database or scanning a whole PDB. This skill reads artifacts; it does not execute target instructions, reconstruct business behavior or claim original Rust parameters/layouts.

Use the project's retained artifact identities and confirmed RVA/range requests. Read [manifest and output contract](references/contract.md) when preparing inputs or consuming results. Run the bundled collector with Python 3.11+ and installed LLVM tools:

```sh
python3 scripts/collect.py manifest.json --output evidence-pack --cache evidence-cache
```

Resolve script paths relative to this skill directory. Explicit tool paths in the manifest take precedence over PATH discovery. Do not silently substitute a different binary/version when files, hashes, debug identity or tools fail.

The collector currently supports file-backed executable ranges in PE x86/x64 and MSF7 PDB identity. It uses one symbolizer process for all uncached address samples, plus one bounded objdump process per distinct uncached range. Other formats require a separate, identity-bound backend; do not reinterpret their addresses as PE RVAs.

Keep these distinctions visible:

- RVA is relative to the original PE image base. objdump receives VA; symbolizer receives RVA with `--relative-address`. A patched analysis database is not the original artifact.
- Whole SHA-256 is checked before cache lookup. A PDB must match PE CodeView GUID/Age and be bound beside the PE under its recorded basename. Ambiguous debug resolution is rejected. Cache identity also includes debug hash, tools' full hashes/versions, request/options and collector revision.
- Every emitted assembly instruction is checked against original file bytes. The tool can finish an instruction whose start is inside the requested range but whose end crosses it; `instruction_overhang_bytes` records this explicitly. The range is not proof of a complete function or CFG.
- Source/inline frames are metadata. They do not contain author source text, locals, original ABI, field layout or a proof that a library wrapper contains no game callback. Unknown source results fail the request by default; an explicit `allow_missing_symbols` retains them as unknown.
- Preserve `report.json` and request JSON/ASM on disk. Send the agent concise identities, paths and unresolved facts, not repeated full dumps.

Use measured `analysis_processes`, `version_processes`, `cache_hits`, elapsed time and output bytes. A warm pack still rehashes artifacts/tools and launches version probes; do not report zero total processes or a hypothetical speed multiplier. Cache integrity failure is an error, not an automatic miss. Correct or remove the specific corrupt entry explicitly before retrying.

Self-check after modifying scripts:

```sh
python3 scripts/test_collect.py
```

Those tests cover pack mechanics with fake tools. Separately smoke one known real original address, rerun unchanged for warm reuse, and save a deliberate identity/range failure. Neither check constitutes business algorithm validation.
