---
name: native-tool-batch
description: Batch bounded native analysis queries through an existing MCP operator, validate nested results and retain compact evidence. Use when repeated function, xref or memory queries incur orchestration overhead; not for whole-program analysis or arbitrary database mutations.
---

# Native Tool Batch

Read the project's tool guide and current schema snapshot. Choose the existing persistent provider for known functions; use LLVM for source/inline points and raw instructions. Do not launch several providers to answer the same question without a concrete disagreement.

Use `scripts/run_batch.py PLAN.json OUTPUT_DIRECTORY`. A plan supplies operator/interpreter paths, backend, worker, pinned artifact and bounded exact-schema steps. See [plan-format.md](references/plan-format.md). The script deduplicates exact steps, runs one operator session, preserves the complete report, parses nested payloads, and writes a small summary. It does not cache mutable database results. Each run rehashes the pinned file; an optional metadata guard checks the selected program's returned executable path. A path/file match is weaker than independently reading the database's import hash; report that distinction.

Explicitly select program/database in every applicable step. Fetch only needed fields, disable caller decompilation unless needed, and limit function batches to 20. Separate identity from oracle: a patched analysis database is not an original instruction oracle. Keep raw evidence on disk; read the summary and selected payload files.

Reject transport errors, nested failure/error fields, missing function entries, failed scripts and placeholder-only decompilation as unresolved. Preserve failed reports. Never interpret `ok: true` with allowed errors as semantic success. Dedicated writable workers and the PyGhidra skill are required for in-session scripts/signature overlays; this batch helper intentionally only permits read queries. Record worker/revision with evidence and reconcile overlays explicitly.

Measure operator process starts, unique MCP steps, result bytes, elapsed time and downstream delivery. A single shell invocation with several MCP steps is not one underlying analysis call. No speedup claim without comparable baseline measurements.
