# Plan format

```json
{
  "interpreter": "/absolute/python",
  "operator": "/absolute/reverse-operator.py",
  "backend": "ghidra",
  "worker": "shared",
  "artifact": {"path": "/absolute/binary", "sha256": "64 lowercase hex digits"},
  "metadata_guard": {"step": 0, "field": "executable_path", "equals": "/absolute/binary"},
  "steps": [{"tool": "get_metadata", "args": {"program": "explicit-name"}, "timeout_seconds": 120}]
}
```

This helper supports persistent Ghidra/ReVa read sessions. Other backends use their existing operator jobs directly. Only known read tool names are allowed. Explicit program/programPath selectors are required in every step; references and ReVa decompilation must specify limit 1..100. Caller context requires explicit caller and line bounds. Single function requests are normalized to bulk mode. Maximum 60 unique steps, timeout per step <=300 seconds. Function bulk requests <=20 with nonempty `fields`. Exact step duplicates are removed in order; guard `step` refers to this unique order. The full catalog stays in operator-report.json. Every text/structured payload is saved under payloads; summary.json contains counters, errors, identity scope and the manifest/source hashes. `allow_error` is rejected. A failure exits nonzero and leaves the failed report. Unknown provider response shape is retained for human inspection; this script does not infer business equivalence.
