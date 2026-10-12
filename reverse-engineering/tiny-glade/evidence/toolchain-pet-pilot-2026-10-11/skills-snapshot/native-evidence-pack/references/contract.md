# Manifest, output and failure contract

## Input

`schema_version: 1`. All paths may be absolute or relative to the manifest directory. No shell expressions are evaluated. Supply one artifact per pack; use separate manifests for versions.

```json
{
  "schema_version": 1,
  "artifact": {
    "id": "application-release",
    "version": "release-label",
    "path": "original.exe",
    "sha256": "FULL_64_HEX_DIGIT_BINARY_SHA256",
    "debug": {
      "path": "original.pdb",
      "sha256": "FULL_64_HEX_DIGIT_DEBUG_SHA256"
    }
  },
  "tools": {
    "symbolizer": "/installed/llvm/bin/llvm-symbolizer",
    "objdump": "/installed/llvm/bin/llvm-objdump"
  },
  "requests": [
    {
      "id": "constructor-entry",
      "rva": "0x1000",
      "length": 32,
      "symbolize_offsets": [0, 4],
      "allow_missing_symbols": false
    }
  ]
}
```

Replace the two hash strings with actual full digests; the collector rejects placeholders. `debug` is mandatory when PE carries CodeView PDB metadata. This version supports PE x86/x64 plus MSF7. It checks PDB Info GUID/Age against PE RSDS and limits debug lookup to the bound sibling PDB; it does not parse type/local streams.

`id` and `version` are nonempty identity labels, never substitutes for hashes. Request IDs are unique path-safe ASCII letters/digits/underscore/hyphen. RVA/length accept integers or 0x strings, not booleans. Limits: 128 requests, 1..65536 bytes each, 32768 total uncached symbol samples. Timeout defaults to30 seconds per process (`--timeout` permits up to300).

Without `symbolize_offsets`, every decoded instruction start is sampled. Explicit offsets are relative to the request start and must be inside its length; they may be inside an instruction because source lookup is point metadata. A start+4 sample is not a complete inline-range reconstruction. Sparse sampling saves cost while keeping its scope clear.

## Output

`report.json` has:

- `schema_version`, `status` (`ok`, `partial`, `error`), `target_executed: false`.
- `artifact`: labels, resolved path, observed full SHA/bytes, PE base, RSDS; matching debug identity when applicable.
- `tools`: resolved paths, actual version text and executable SHA.
- `requests`: ID/status/cache key/hit, evidence+assembly absolute paths and sample/instruction counts.
- `tool_invocations`: exact argv, phase, exit status/timeout, elapsed seconds and output-byte counts; diagnostics preserved.
- `stats`: actual requests/unique requests, hits/misses, uncached sample points, evidence bytes, process totals split into analysis/version, tool stdout/stderr bytes and total elapsed seconds.
- `error` for global identity/format/range/tool/cache/parser failure.

Each `<request-id>.json` contains artifact identity, cache key, range, verified original-byte instructions and LLVM JSON symbolization records. Source frames preserve original `FunctionName`, `FileName`, `Line`, `Column` and other LLVM fields. `missing_symbol_addresses` explicitly lists unresolved samples. `range.function_boundary_verified` stays false: this tool is not a PDB procedure/CFG boundary validator.

`<request-id>.asm.txt` preserves bounded objdump stdout. `range.instruction_overhang_bytes` identifies a final whole instruction crossing the requested stop (up to14 bytes on supported x86/x64). Instruction starts are all within [start,stop); bytes extending the stop are verified too. Choose instruction-aligned stops if no overhang is desired. Requested ranges outside file-backed executable sections fail before tool execution.

`evidence_output_bytes` counts request JSON/ASM bytes, excluding report/cache overhead. `tool_stdout_bytes` measures actual external-tool stdout, not cached payload bytes. Warm version probes are real processes and included in totals. Same-content duplicate requests in one pack are counted as in-run reuse; unique_requests is the number of distinct keys. No timing claims are derived from these counts.

Cache files contain exact bindings and payload SHA; each request/options, artifact+debug identity, full tool identity and collector source hash contribute to key. Renaming IDs can reuse content; moving artifact paths conservatively changes identity. Valid negative source results can be cached but remain partial/error unless the explicit missing-source option permits unknown metadata. Timeouts and tool/parser failures never become success cache entries.

## Failure and retry

Exit0 requires every request status `ok`; exit2 means partial/error. Always inspect the report instead of relying only on process return code. Hash mismatch, unmatched PDB, unbound debug resolver, tool diagnostics/nonzero return, timeout, malformed JSON, wrong returned address, empty/gapped/byte-mismatching disassembly and corrupt cache fail closed. They do not produce guessed names/types or fall back to another baseline.

Failures retain diagnostics and known metrics. Fix the identity/request/tool input before retrying. A corrupt cache requires deliberate removal of that exact file, not transparent replacement. Keep release artifacts immutable; output and cache directories are separate from the release tree. This script uses no target execution, network debug download, database mutation or compiler invocation.
