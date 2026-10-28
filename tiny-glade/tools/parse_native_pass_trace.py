"""Scope-only diagnostic; never a clean CPU benchmark or submission receipt."""
import argparse
import ast
from collections import defaultdict
import json
import re
from pathlib import Path

FIELDS = (
    "direct_draw_api_calls", "indexed_draw_api_calls", "indirect_draw_api_calls",
    "indirect_count_draw_api_calls", "indexed_indirect_draw_api_calls",
    "indexed_indirect_count_draw_api_calls", "direct_dispatch_api_calls",
    "indirect_dispatch_api_calls", "staging_h2d_copy_recorded_bytes",
    "immutable_producer_h2d_copy_recorded_bytes", "cached_d2d_copy_recorded_bytes",
)
RECORD = re.compile(
    r'RHAPSODY_RECORD_TRACE id=(\d+) graph=(\d+) pass=(\d+) command=(\d+) '
    r'name=("(?:\\.|[^"\\])*") callback_ok=(Some\((?:true|false)\)|None) '
    r'unwind=(true|false).*? native_workload=(None|Some\(CommandWorkload \{(.*?)\}\))\s*$'
)


def parse_workload(body):
    fields = {}
    for item in body.split(","):
        key, value = (part.strip() for part in item.split(":", 1))
        if key in fields:
            raise ValueError("duplicate native field")
        if key == "valid":
            if value not in ("true", "false"):
                raise ValueError("invalid native boolean")
            fields[key] = value == "true"
        else:
            if key not in FIELDS or not re.fullmatch(r"[0-9]+", value):
                raise ValueError("invalid native integer or unknown field")
            fields[key] = int(value)
            if fields[key] > (1 << 64) - 1:
                raise ValueError("native counter exceeds u64")
    if set(fields) != set(FIELDS) | {"valid"}:
        raise ValueError("missing native fields")
    return fields


def parse_trace(text):
    records, chunks, errors = [], [], []
    pending = defaultdict(list)
    ids = set()
    totals = dict.fromkeys(FIELDS, 0)
    for number, line in enumerate(text.splitlines(), 1):
        if "RHAPSODY_RECORD_TRACE " in line:
            match = RECORD.search(line)
            if not match:
                errors.append({"line": number, "error": "malformed_or_old_record_schema"})
                continue
            scope_id, graph, pass_index, command = map(int, match.group(1, 2, 3, 4))
            try:
                name = json.loads(match.group(5))
                workload = None if match.group(8) == "None" else parse_workload(match.group(9))
            except (ValueError, TypeError) as error:
                errors.append({"line": number, "error": "malformed_record_payload", "detail": str(error)})
                continue
            valid = (scope_id != 0 and scope_id not in ids and match.group(6) == "Some(true)"
                     and match.group(7) == "false" and workload is not None and workload["valid"])
            if scope_id in ids:
                errors.append({"line": number, "error": "duplicate_scope_id", "scope_id": scope_id})
            ids.add(scope_id)
            record = {"line": number, "scope_id": scope_id, "graph": graph, "pass": pass_index,
                      "command": command, "name": name, "callback_ok": match.group(6),
                      "unwind": match.group(7) == "true", "workload": workload, "valid": valid}
            records.append(record)
            pending[command].append(record)
            if not valid:
                errors.append({"line": number, "error": "incomplete_or_failed_record", "scope_id": scope_id})
        elif "RHAPSODY_SUBMIT_TRACE scope=record_owner_transfer " in line:
            try:
                pairs = ast.literal_eval(line.split(" counts=", 1)[1])
                counts = dict(pairs)
                if len(counts) != len(pairs) or not all(type(v) is int and v >= 0 for v in counts.values()):
                    raise ValueError("duplicate or invalid chunk field")
                required = {"command", "graph", "direct_dispatch_api_calls", "native_valid", "recorded_passes"}
                if not required <= counts.keys() or counts["native_valid"] not in (0, 1):
                    raise ValueError("missing or invalid chunk fields")
            except (SyntaxError, ValueError, IndexError, TypeError) as error:
                errors.append({"line": number, "error": "malformed_or_old_chunk_schema", "detail": str(error)})
                continue
            rows = pending.pop(counts["command"], [])
            # Trace is emitted in worker completion order. Keep original scope
            # IDs and sort each chunk by true pass index, never by display name.
            rows = sorted(rows, key=lambda row: (row["pass"], row["scope_id"]))
            summed = {field: sum(row["workload"][field] for row in rows if row["workload"] is not None)
                      for field in FIELDS}
            valid = (counts["native_valid"] == 1 and len(rows) == counts["recorded_passes"]
                     and all(row["valid"] for row in rows)
                     and (not rows or rows[0]["graph"] == counts["graph"])
                     and len({row["pass"] for row in rows}) == len(rows)
                     and summed["direct_dispatch_api_calls"] == counts["direct_dispatch_api_calls"]
                     and all(value <= (1 << 64) - 1 for value in summed.values()))
            if not valid:
                errors.append({"line": number, "error": "chunk_invalid_or_dispatch_not_conserved"})
            chunks.append({"line": number, "ordinal": len(chunks), "graph": counts["graph"],
                           "command": counts["command"], "counts": counts,
                           "scope_ids": [row["scope_id"] for row in rows], "pass_sum": summed, "valid": valid})
            for field in FIELDS:
                totals[field] += summed[field]
    for command, rows in pending.items():
        errors.append({"error": "unclosed_chunk", "command": command,
                       "scope_ids": [row["scope_id"] for row in rows]})
    if not records or not chunks:
        errors.append({"error": "no_complete_observations"})
    return {"schema": "native-pass-trace-v1", "measurement_class": "trace_only_diagnostic",
            "clean_cpu_acceptance_eligible": False, "submission_receipt_association": "not_available_scope_only",
            "run_valid": None, "trace_valid": not errors, "records": records, "chunks": chunks,
            "raw_pass_totals": totals, "errors": errors,
            "scope": "actual recorded callbacks; successful recording is not proof of GPU submission; includes warmup/startup",
            "conservation": "direct dispatch and pass count checked per command; other fields summed without independent chunk vector"}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("log", type=Path)
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    report = parse_trace(args.log.read_text(encoding="utf-8", errors="strict"))
    args.output.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8", newline="\n")
    return 0 if report["trace_valid"] else 2


if __name__ == "__main__":
    raise SystemExit(main())
